use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use anyhow::{Error, anyhow};
use log::{info, error};

use freepdk_gen::{
    mcu::{Frequency, Port, Pin, StopBits},
    uart::UartGenerator,
};

#[wasm_bindgen]
struct UartGeneratorApp {
    mcu_clock_frequency_control: web_sys::HtmlInputElement,
    uart_baud_control: web_sys::HtmlInputElement,
    tx_port_control: web_sys::HtmlSelectElement,
    tx_pin_control: web_sys::HtmlSelectElement,
    rx_port_control: web_sys::HtmlSelectElement,
    rx_pin_control: web_sys::HtmlSelectElement,
    tx_inverted_control: web_sys::HtmlInputElement,
    rx_inverted_control: web_sys::HtmlInputElement,
    uart_num_control: web_sys::HtmlInputElement,
    stop_bits_control: web_sys::HtmlSelectElement,
    code_block_container: web_sys::HtmlDivElement,
    code_block: web_sys::Element,

    result: Option<String>,
}

#[wasm_bindgen]
impl UartGeneratorApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());

        let window = web_sys::window().expect("Failed to get window object");
        let document = window.document().expect("Failed to get document object");

        let mcu_clock_frequency_control: web_sys::HtmlInputElement =
            get_typed_element_by_id(&document, "uart-gen-mcu-frequency-control");
        let uart_baud_control: web_sys::HtmlInputElement =
            get_typed_element_by_id(&document, "uart-gen-baud-control");
        let tx_port_control: web_sys::HtmlSelectElement =
            get_typed_element_by_id(&document, "uart-gen-tx-port-control");
        let tx_pin_control: web_sys::HtmlSelectElement =
            get_typed_element_by_id(&document, "uart-gen-tx-pin-control");
        let rx_port_control: web_sys::HtmlSelectElement =
            get_typed_element_by_id(&document, "uart-gen-rx-port-control");
        let rx_pin_control: web_sys::HtmlSelectElement =
            get_typed_element_by_id(&document, "uart-gen-rx-pin-control");
        let tx_inverted_control: web_sys::HtmlInputElement =
            get_typed_element_by_id(&document, "uart-gen-invert-tx-control");
        let rx_inverted_control: web_sys::HtmlInputElement =
            get_typed_element_by_id(&document, "uart-gen-invert-rx-control");
        let uart_num_control: web_sys::HtmlInputElement =
            get_typed_element_by_id(&document, "uart-gen-number-control");
        let stop_bits_control: web_sys::HtmlSelectElement =
            get_typed_element_by_id(&document, "uart-gen-stop-bits-control");
        let code_block_container: web_sys::HtmlDivElement =
            get_typed_element_by_id(&document, "uart-gen-code-block-container");
        let code_block: web_sys::Element =
            get_typed_element_by_id(&document, "uart-gen-code-block");

        Self {
            mcu_clock_frequency_control,
            uart_baud_control,
            tx_port_control,
            tx_pin_control,
            rx_port_control,
            rx_pin_control,
            tx_inverted_control,
            rx_inverted_control,
            uart_num_control,
            stop_bits_control,
            code_block_container,
            code_block,

            result: None
        }
    }

    pub fn on_submit(&mut self) {
        self.submit().unwrap_or_else(|e| {
            // TODO: normal error handling
            error!("Generation failed: {}", e);
        })
    }

    pub fn get_result(&self) -> String {
        self.result.clone().unwrap_or_default()
    }
}

impl UartGeneratorApp {
    fn build_uart_generator(&self) -> Result<UartGenerator, Error> {
        let frequency: Frequency = self.mcu_clock_frequency_control
            .value().parse().map_err(|e| anyhow!("{}", e))?;
        let baud: u32 = self.uart_baud_control
            .value().parse().map_err(|e| anyhow!("{}", e))?;
        let tx_port: Port = get_option_string_value(&self.tx_port_control)
            .parse().map_err(|e| anyhow!("{}", e))?;
        let tx_pin: Pin = get_option_string_value(&self.tx_pin_control)
            .parse().map_err(|e| anyhow!("{}", e))?;
        let rx_port: Port = get_option_string_value(&self.rx_port_control)
            .parse().map_err(|e| anyhow!("{}", e))?;
        let rx_pin: Pin = get_option_string_value(&self.rx_pin_control)
            .parse().map_err(|e| anyhow!("{}", e))?;
        let tx_inverted =  self.tx_inverted_control
            .checked();
        let rx_inverted = self.rx_inverted_control
            .checked();
        let uart_number: u8 = self.uart_num_control
            .value().parse().map_err(|e| anyhow!("{}", e))?;
        let stop_bits: StopBits = get_option_string_value(&self.stop_bits_control)
            .parse().map_err(|e| anyhow!("{}", e))?;

        let mut builder = UartGenerator::builder()
            .frequency(frequency)
            .baud(baud)
            .tx_port(tx_port)
            .tx_pin(tx_pin)
            .rx_port(rx_port)
            .rx_pin(rx_pin)
            .uart_num(uart_number)
            .stop_bits(stop_bits);
        if tx_inverted {
            builder = builder.invert_tx();
        }
        if rx_inverted {
            builder = builder.invert_rx();
        }

        let generator = builder.build()?;

        info!("MCU frequency: {}", frequency);
        info!("UART baud: {}", baud);
        info!("TX pin: P{}{}", tx_port.char(), tx_pin.num());
        info!("RX pin: P{}{}", rx_port.char(), rx_pin.num());
        info!("TX inverted: {}", tx_inverted);
        info!("RX inverted: {}", rx_inverted);
        info!("UART number: {}", uart_number);
        info!("Stop bits: {:?}", stop_bits);

        Ok(generator)
    }

    fn submit(&mut self) -> Result<(), Error> {
        info!("Building UART code generator...");
        let generator = self.build_uart_generator()?;

        info!("Generating uart code...");
        let generated = generator.generate()?;

        self.code_block_container.set_hidden(false);
        self.code_block.set_inner_html(&generated);
        
        self.result.replace(generated);

        info!("Uart code generation successfully completed!");
        Ok(())
    }
}


fn get_typed_element_by_id<T: JsCast>(document: &web_sys::Document, id: &str) -> T {
    document
        .get_element_by_id(id)
        .expect(&format!("Can't find {} element", id))
        .dyn_into::<T>()
        .expect(&format!("{} element has invalid type", id))
}

fn get_option_string_value(option: &web_sys::HtmlSelectElement) -> String {
    let selected_index = option.selected_index();
    option
        .options()
        .item(selected_index as u32)
        .expect("Select index out of bounds")
        .dyn_into::<web_sys::HtmlOptionElement>()
        .expect("Invalid option element type")
        .text()
}
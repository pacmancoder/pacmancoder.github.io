use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use anyhow::{Error, anyhow};
use log::{info, error};

use crate::frontend::{UartGeneratorFrontend, UartGeneratorErrorEvent, UartGeneratorFrontendError};
use freepdk_gen::mcu::{StopBits, Pin, Port, Frequency};

const HTML_CLASS_IS_INVALID: &str = "is-invalid";

pub struct UartGeneratorWebFrontend {
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
    error_alert: web_sys::HtmlDivElement,
}

impl UartGeneratorWebFrontend {
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
        let error_alert: web_sys::HtmlDivElement =
            get_typed_element_by_id(&document, "uart-get-error-alert");

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
            error_alert,
        }
    }
}

/*
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
 */

impl UartGeneratorFrontend for UartGeneratorWebFrontend {
    fn frequency(&self) -> Result<Frequency, UartGeneratorFrontendError> {
        self.mcu_clock_frequency_control.value().parse::<Frequency>().map_err(|e| {
            self.mcu_clock_frequency_control.class_list().add_1(HTML_CLASS_IS_INVALID);
            UartGeneratorFrontendError(e.to_string())
        })
    }

    fn baud(&self) -> Result<u32, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn tx_port(&self) -> Result<Port, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn tx_pin(&self) -> Result<Pin, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn rx_port(&self) -> Result<Port, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn rx_pin(&self) -> Result<Pin, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn tx_inverted(&self) -> Result<bool, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn rx_inverted(&self) -> Result<bool, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn uart_number(&self) -> Result<u8, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn stop_bits(&self) -> Result<StopBits, UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn reset(&mut self) -> Result<(), UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn show_result(&mut self) -> Result<(), UartGeneratorFrontendError> {
        unimplemented!()
    }

    fn show_error(&mut self, error: &UartGeneratorErrorEvent) -> Result<(), UartGeneratorFrontendError> {
        unimplemented!()
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
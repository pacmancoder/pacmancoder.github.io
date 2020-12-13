use freepdk_gen::{
    mcu::{
        Frequency,
        Pin,
        Port,
        StopBits,
    },
};
use thiserror::Error;
use wasm_bindgen::__rt::std::process::Output;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct UartGeneratorFrontendError(pub String);

impl From<T> for UartGeneratorFrontendError where T: ToOwned<Output = Self> {
    fn from(e: T) -> Self {
        Self(e.to_owned())
    }
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct UartGeneratorAppError(pub String);

impl From<T> for UartGeneratorAppError where T: ToOwned<Output = Self> {
    fn from(e: T) -> Self {
        Self(e.to_owned())
    }
}

pub enum UartGeneratorErrorEvent {
    FrontendError(UartGeneratorFrontendError),
    AppError(UartGeneratorAppError),
    GeneratorError(freepdk_gen::uart::Error),
}

impl From<UartGeneratorFrontendError> for UartGeneratorErrorEvent {
    fn from(e: UartGeneratorFrontendError) -> Self {
        Self::FrontendError(e)
    }
}

impl From<UartGeneratorAppError> for UartGeneratorErrorEvent {
    fn from(e: UartGeneratorAppError) -> Self {
        Self::AppError(e)
    }
}

impl From<freepdk_gen::uart::Error> for UartGeneratorErrorEvent {
    fn from(e: freepdk_gen::uart::Error) -> Self {
        Self::GeneratorError(e)
    }
}

pub trait UartGeneratorFrontend {
    fn frequency(&self) -> Result<Frequency, UartGeneratorFrontendError>;
    fn baud(&self) -> Result<u32, UartGeneratorFrontendError>;
    fn tx_port(&self) -> Result<Port, UartGeneratorFrontendError>;
    fn tx_pin(&self) -> Result<Pin, UartGeneratorFrontendError>;
    fn rx_port(&self) -> Result<Port, UartGeneratorFrontendError>;
    fn rx_pin(&self) -> Result<Pin, UartGeneratorFrontendError>;
    fn tx_inverted(&self) -> Result<bool, UartGeneratorFrontendError>;
    fn rx_inverted(&self) -> Result<bool, UartGeneratorFrontendError>;
    fn uart_number(&self) -> Result<u8, UartGeneratorFrontendError>;
    fn stop_bits(&self) -> Result<StopBits, UartGeneratorFrontendError>;

    fn reset(&mut self) -> Result<(), UartGeneratorFrontendError>;
    fn show_result(&mut self) -> Result<(), UartGeneratorFrontendError>;
    fn show_error(&mut self, error: &UartGeneratorErrorEvent) -> Result<(), UartGeneratorFrontendError>;
}
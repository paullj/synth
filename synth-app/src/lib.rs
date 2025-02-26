pub mod app;
pub mod input;

pub(crate) mod mode;
pub(crate) mod screen;

// Only compile this module on the Raspberry Pi
#[cfg(feature = "raspberry_pi")]
pub mod spi;

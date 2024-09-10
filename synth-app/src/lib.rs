pub mod app;

// Only compile this module on the Raspberry Pi
#[cfg(feature = "raspberry_pi")]
pub mod spi;

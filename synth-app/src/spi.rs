use embedded_hal::spi::{ErrorType, Operation, SpiDevice};
use rppal::spi::Spi;

/// Wrapper around rppal's Spi struct to implement the SpiDevice trait.
pub struct SpiWrapper {
    pub spi: Spi,
}

impl ErrorType for SpiWrapper {
    type Error = rppal::spi::Error;
}

/// Implement the SpiDevice trait for SpiWrapper, since rppal doesn't seem to implement it.
impl SpiDevice for SpiWrapper {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                Operation::Read(buf) => {
                    let _ = self.spi.read(buf).unwrap();
                }
                Operation::Write(buf) => {
                    let _ = self.spi.write(buf).unwrap();
                }
                Operation::Transfer(rd, wr) => {
                    let _ = self.spi.transfer(rd, wr).unwrap();
                }
                // TODO: Figure out what to do here, rppal doesn't seem to support these operations
                // embedded_hal::spi::Operation::TransferInPlace(buf) => {
                //     self.spi.transfer(buf)?
                // }
                // embedded_hal::spi::Operation::DelayNs(_) => ()
                _ => (),
            }
        }
        Ok(())
    }
}

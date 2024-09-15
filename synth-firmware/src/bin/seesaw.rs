#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy_executor::Spawner;

use adafruit_seesaw::{devices::RotaryEncoder, prelude::*, SeesawRefCell};
use embassy_rp::i2c::{Config, I2c};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());
    let sda = peripherals.PIN_0;
    let scl = peripherals.PIN_1;
    let config = Config::default();

    let i2c = I2c::new_blocking(peripherals.I2C0, scl, sda, config);

    let seesaw = SeesawRefCell::new(embassy_time::Delay, i2c);

    let mut encoder = RotaryEncoder::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start RotaryEncoder");

    defmt::debug!("Looping...");
    loop {
        let delta = encoder.delta().expect("Failed to get delta");
        let position = encoder.position().expect("Failed to get position");
        // let c = color_wheel(((position & 0xFF) as u8).wrapping_mul(3));
        // let Color(r, g, b) = c.set_brightness(50);
        defmt::debug!("Position: {}, Delta {}", position, delta);

        // encoder
        //     .set_neopixel_color(r, g, b)
        //     .and_then(|_| encoder.sync_neopixel())
        //     .expect("Failed to set neopixel");

        if let Ok(true) = encoder.button() {
            encoder.set_position(0).expect("Failed to set position");
        }
    }
}

fn color_wheel(byte: u8) -> Color {
    match byte {
        0..=84 => Color(255 - byte * 3, 0, byte * 3),
        85..=169 => Color(0, (byte - 85) * 3, 255 - (byte - 85) * 3),
        _ => Color((byte - 170) * 3, 255 - (byte - 170) * 3, 0),
    }
}

struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn set_brightness(self, brightness: u8) -> Self {
        Self(
            ((self.0 as u16 * brightness as u16) >> 8) as u8,
            ((self.1 as u16 * brightness as u16) >> 8) as u8,
            ((self.2 as u16 * brightness as u16) >> 8) as u8,
        )
    }
}

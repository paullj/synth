use display_interface_spi::SPIInterface;
use embedded_graphics::{
    framebuffer::{self, buffer_size},
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::{
        self,
        raw::{LittleEndian, RawU16},
        Rgb565,
    },
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
    Drawable,
};
use ili9341::{DisplaySize240x320, Ili9341, Orientation};
use rppal::gpio::Gpio;
use rppal::spi::{Bus, SlaveSelect, Spi};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use synth_app::{app::App, spi::SpiWrapper};

use log::{info, warn};

const DC_PIN: u8 = 25;
const RESET_PIN: u8 = 27;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting display test...");
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

    // Setup peripherals
    let gpio = Gpio::new().unwrap();
    let spi = Spi::new(
        Bus::Spi0,
        SlaveSelect::Ss0,
        16_000_000,
        rppal::spi::Mode::Mode0,
    )
    .unwrap();

    // Setup GPIO for Data/Command (DC) and Reset
    let dc_pin = gpio.get(DC_PIN).unwrap().into_output();
    let reset_pin = gpio.get(RESET_PIN)?.into_output();

    let spii = SPIInterface::new(SpiWrapper { spi }, dc_pin);

    // Initialize ILI9341 display
    let mut display = Ili9341::new(
        spii,
        reset_pin,
        &mut rppal::hal::Delay,
        Orientation::Landscape,
        DisplaySize240x320,
    )
    .unwrap();

    let mut app = App::new(&mut display);

    while !term.load(Ordering::Relaxed) {
        app.draw(&mut display)?;
    }

    warn!("Received SIGTERM kill signal. Exiting...");

    Ok(())
}

use adafruit_seesaw::{
    bus::RefCellBus,
    devices::RotaryEncoder,
    prelude::{EncoderModule, SeesawDevice, SeesawDeviceInit},
    SeesawRefCell,
};
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
use embedded_hal::{
    delay::{self, DelayNs},
    i2c::I2c as HalI2c,
};
use ili9341::{DisplaySize240x320, Ili9341, Orientation};
use rppal::gpio::Gpio;
use rppal::i2c::I2c as RppalI2c;
use rppal::spi::{Bus, SlaveSelect, Spi};
use std::{
    borrow::Borrow,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use synth_app::{
    app::App,
    input::{Encoder, Input, InputEvent},
    spi::SpiWrapper,
};

use log::{info, warn};

const DC_PIN: u8 = 25;
const RESET_PIN: u8 = 27;

const ENCODER_A_ADDR: u8 = 0x36;
const ENCODER_B_ADDR: u8 = 0x37;
const ENCODER_C_ADDR: u8 = 0x38;
const ENCODER_D_ADDR: u8 = 0x39;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting display test...");

    // Setup peripherals
    let gpio = Gpio::new().unwrap();
    let spi = Spi::new(
        Bus::Spi0,
        SlaveSelect::Ss0,
        16_000_000,
        rppal::spi::Mode::Mode0,
    )
    .unwrap();
    let i2c = RppalI2c::new().unwrap();

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

    let input = PiInput::new(ENCODER_A_ADDR, rppal::hal::Delay, i2c);

    let mut app = App::new(input);
    // let mut app = App::new(&mut display);

    let run_future = app.run();

    // while !term.load(Ordering::Relaxed) {
    //     let delta = match encoder_a.delta() {
    //         Ok(value) => value,
    //         Err(error) => {
    //             warn!("Failed to read encoder A delta {:?}", error);
    //             0
    //         }
    //     };

    //     let inputs = if delta > 0 {
    //         vec![Input::IncrementEncoder(Encoder::A)]
    //     } else if delta < 0 {
    //         vec![Input::DecrementEncoder(Encoder::A)]
    //     } else {
    //         vec![]
    //     };
    //     app.update(inputs);
    //     app.draw(&mut display)?;
    // }

    // warn!("Received SIGTERM kill signal. Exiting...");

    Ok(())
}

struct PiInput<DELAY, I2C>
where
    DELAY: DelayNs + 'static,
    I2C: HalI2c + 'static,
{
    // seesaw: SeesawRefCell<(DELAY, I2C)>,
    encoder: RotaryEncoder<
        adafruit_seesaw::bus::Bus<'static, adafruit_seesaw::bus::RefCellBus<(DELAY, I2C)>>,
    >,
}

impl<DELAY, I2C> PiInput<DELAY, I2C>
where
    DELAY: DelayNs + 'static,
    I2C: HalI2c + 'static,
{
    pub fn new(address: u8, delay: DELAY, i2c: I2C) -> Self {
        let seesaw = SeesawRefCell::new(delay, i2c);
        let driver = seesaw.acquire_driver();

        let encoder = RotaryEncoder::new(address, seesaw.acquire_driver())
            .init()
            .expect("Failed to start encoder A");

        Self { encoder }
    }
}

impl<DELAY, I2C> Input for PiInput<DELAY, I2C>
where
    DELAY: DelayNs + 'static,
    I2C: HalI2c + 'static,
{
    fn poll_input(&mut self) -> Option<Vec<InputEvent>> {
        let delta = match self.encoder.delta() {
            Ok(value) => value,
            Err(error) => {
                warn!("Failed to read encoder A delta {:?}", error);
                0
            }
        };

        let inputs = if delta > 0 {
            vec![InputEvent::IncrementEncoder(Encoder::A)]
        } else if delta < 0 {
            vec![InputEvent::DecrementEncoder(Encoder::A)]
        } else {
            return None;
        };
        Some(inputs)
    }
}

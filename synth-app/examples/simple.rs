mod constants;

use std::{thread, time::Duration};

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Sample,
};
use sfsm::*;

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

fn main() -> Result<(), std::convert::Infallible> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Couldn't open output device");

    println!("Default output device: {}", device.name().unwrap());

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    println!("Supported audio format: {:?}", sample_format);
    println!("Supported audio config: {:?}", config);

    match sample_format {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()).unwrap(),
        _ => panic!("Unsupported format"),
    }
    Ok(())
}

struct Startup {
    display: SimulatorDisplay<Rgb565>,
    window: Window,
}
struct Synth {
    display: SimulatorDisplay<Rgb565>,
    window: Window,
}

add_state_machine!(
    ApplicationState,                          // Name of the state machine. Accepts a visibility modifier.
    Startup,                   // The initial state the state machine will start in
    [Startup, Synth],         // All possible states
    [
        Startup => Synth,     // All transitions
    ]
);

impl State for Startup {
    fn entry(&mut self) {}

    fn execute(&mut self) {
        let duration = Duration::from_millis(1000);
        let step = Duration::from_millis(100);
        let mut elapsed = Duration::from_millis(0);
        'running: loop {
            self.display.clear(constants::color::WHITE);

            // DRAW

            self.window.update(&self.display);

            for event in self.window.events() {
                match event {
                    SimulatorEvent::KeyDown {
                        keycode,
                        repeat: false,
                        ..
                    } => match keycode {
                        _ => {
                            println!("Key pressed: {:?}", keycode);
                            None::<()>
                        }
                    },
                    SimulatorEvent::Quit => break 'running,
                    _ => None,
                };
            }
            if elapsed >= duration {
                break 'running;
            }
            thread::sleep(step);
            elapsed += step;
        }
    }

    fn exit(&mut self) {}
}
impl State for Synth {
    fn entry(&mut self) {}

    fn execute(&mut self) {
        let duration = Duration::from_millis(1000);
        let step = Duration::from_millis(100);
        let mut elapsed = Duration::from_millis(0);
        'running: loop {
            self.display.clear(constants::color::BLACK);

            // DRAW

            self.window.update(&self.display);

            for event in self.window.events() {
                match event {
                    SimulatorEvent::KeyDown {
                        keycode,
                        repeat: false,
                        ..
                    } => match keycode {
                        _ => {
                            println!("Key pressed: {:?}", keycode);
                            None::<()>
                        }
                    },
                    SimulatorEvent::Quit => break 'running,
                    _ => None,
                };
            }
            if elapsed >= duration {
                break 'running;
            }
            thread::sleep(step);
            elapsed += step;
        }
    }

    fn exit(&mut self) {
    }
}

impl Transition<Synth> for Startup {
    fn guard(&self) -> TransitGuard {
        TransitGuard::Transit
    }
}

impl From<Startup> for Synth {
    fn from(startup: Startup) -> Self {
        Synth {
            display: startup.display,
            window: startup.window,
        }
    }
}

#[sfsm_trace]
fn trace(log: &str) {
    println!("{}", log);
}

fn run<T: Sample>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error> {
    // Create a new simulator display for local development
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        constants::display::WIDTH,
        constants::display::HEIGHT,
    ));

    // Create a new window
    let output_settings = OutputSettingsBuilder::new().scale(4).max_fps(60).build();
    let mut window = Window::new("simulator", &output_settings);

    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let mut state_machine = ApplicationState::new();
    let _ = state_machine.start(Startup {
        display: display,
        window: window,
    });

    let _ = state_machine.step();
    let _ = state_machine.step();

    let _ = state_machine.stop();

    Ok(())
}

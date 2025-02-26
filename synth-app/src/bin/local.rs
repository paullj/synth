use anyhow::Error;
use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics_simulator::{sdl2, OutputSettingsBuilder, SimulatorDisplay, Window};
use synth_app::{
    app::App,
    input::{InputEvent, Keycode},
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const SCALE: u32 = 1;
const MAX_FPS: u32 = 60;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(WIDTH, HEIGHT));

    let output_settings = OutputSettingsBuilder::new()
        .scale(SCALE)
        .max_fps(MAX_FPS)
        .build();

    let mut window = Window::new("simulator", &output_settings);
    window.update(&display);

    let mut app = App::new(&mut display);

    loop {
        // Get inputs
        let inputs = match get_inputs(&mut window) {
            Ok(inputs) => inputs,
            Err(_) => return Ok(()),
        };

        app.update(inputs);
        // Draw the app
        let _ = app.draw(&mut display);

        // Update simulator display
        window.update(&display);
    }
}

fn get_inputs(window: &mut Window) -> Result<Vec<InputEvent>, ()> {
    let mut inputs = vec![];
    for e in window.events() {
        match e {
            embedded_graphics_simulator::SimulatorEvent::Quit => return Err(()),
            embedded_graphics_simulator::SimulatorEvent::KeyDown {
                keycode,
                keymod,
                repeat,
            } => {
                if !repeat {
                    if let Some(keycode) = keycode_to_keycode(keycode) {
                        inputs.push(InputEvent::KeyDown(keycode));
                    }
                }
            }
            embedded_graphics_simulator::SimulatorEvent::KeyUp {
                keycode,
                keymod,
                repeat,
            } => {
                if !repeat {
                    if let Some(keycode) = keycode_to_keycode(keycode) {
                        inputs.push(InputEvent::KeyUp(keycode));
                    }
                }
            }
            _ => (),
        }
    }
    Ok(inputs)
}

fn keycode_to_keycode(keycode: sdl2::Keycode) -> Option<Keycode> {
    let new_keycode = match keycode {
        sdl2::Keycode::Q => Keycode::A,
        sdl2::Keycode::W => Keycode::B,
        sdl2::Keycode::O => Keycode::X,
        sdl2::Keycode::P => Keycode::Y,
        _ => return None,
    };
    Some(new_keycode)
}

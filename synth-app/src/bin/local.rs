use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use synth_app::app::App;

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
        for e in window.events() {
            match e {
                embedded_graphics_simulator::SimulatorEvent::Quit => return Ok(()),
                _ => {}
            }
        }

        let _ = app.draw(&mut display);
        window.update(&display);
    }
}

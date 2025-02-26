use embedded_fps::{StdClock, FPS};
use embedded_graphics::{
    framebuffer::{buffer_size, Framebuffer},
    mono_font::{self, MonoTextStyleBuilder},
    pixelcolor::{
        raw::{LittleEndian, RawU16},
        Rgb565,
    },
    prelude::*,
    primitives::Rectangle,
    text::Text,
};

use crate::{
    input::{Input, InputEvent},
    screen::Screen,
};
use crate::{
    mode::Event,
    screen::{startup::StartupScreen, track::TrackScreen},
};
use crate::{mode::Mode, screen::synth::SynthScreen};

pub struct App<I>
where
    I: Input,
{
    // TODO: Make this generic over the color type, and buffer size
    // buffer:
    //     Framebuffer<Rgb565, RawU16, LittleEndian, 320, 240, { buffer_size::<Rgb565>(320, 240) }>,
    // bounding_box: Rectangle,
    // fps_counter: FPS<1000, StdClock>,
    // fps: usize,
    mode: Mode,
    input: I,
}

impl<I> App<I>
where
    I: Input,
{
    // pub fn new<'a, D>(display: &'a mut D) -> Self
    // where
    //     D: DrawTarget,
    //     D::Color: RgbColor + From<RawU16>,
    // {
    //     env_logger::init();

    //     let term = Arc::new(AtomicBool::new(false));
    //     signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

    //     let _ = display.clear(D::Color::BLACK);

    //     let bounding_box = display.bounding_box();
    //     let buffer = Framebuffer::<
    //         Rgb565,
    //         _,
    //         LittleEndian,
    //         320,
    //         240,
    //         { buffer_size::<Rgb565>(320, 240) },
    //     >::new();

    //     let midi_input = midir::MidiInput::new("MIDI Input").unwrap();
    //     let mut device_port: Option<midir::MidiInputPort> = None;

    //     for port in midi_input.ports() {
    //         if let Ok(port_name) = midi_input.port_name(&port) {
    //             println!("Port: {}", port_name);
    //         }
    //     }
    //     let mut fps_counter = FPS::<1000, StdClock>::default();

    //     let mut initial_screen = StartupScreen::default();
    //     initial_screen.entry();

    //     Self {
    //         buffer,
    //         bounding_box,
    //         mode: Mode::Startup(initial_screen),
    //         fps_counter,
    //         fps: 0,
    //     }
    // }

    pub fn new(input: I) -> Self {
        Self {
            input,
            mode: Mode::Startup(StartupScreen::default()),
        }
    }

    pub(crate) fn next(&mut self, event: Event) {
        let next_mode = match (&self.mode, event) {
            (Mode::Startup(_), Event::Initialized) => Some(Mode::Synth(SynthScreen::default())),
            (_, Event::Quit) => None,
            (_, Event::Error(message)) => Some(Mode::Error(message)),
            (_, _) => Some(Mode::Error("Unknown error from unhandled event".into())),
        };

        if let Some(next_mode) = next_mode {
            self.handle_exit();
            self.mode = next_mode;
            self.handle_entry();
        }
    }

    // pub fn update(&mut self, inputs: Vec<InputEvent>) {
    //     let ref mut mode = self.mode;
    //     let event = match mode {
    //         Mode::Startup(screen) => screen.update(inputs),
    //         Mode::Synth(screen) => screen.update(inputs),
    //         Mode::Track(screen) => screen.update(inputs),
    //         Mode::Error(screen) => None,
    //     };
    //     if let Some(event) = event {
    //         self.next(event);
    //     }

    //     // for input in inputs {
    //     //     match input {
    //     //         Input::KeyDown(key) => {
    //     //             println!("Key down: {:?}", key);
    //     //             self.mode = match &self.mode {
    //     //                 // Mode::Startup(screen) => Mode::Track(TrackScreen::default()),
    //     //                 // Mode::Synth(screen) => Mode::Track(TrackScreen::default()),
    //     //                 // Mode::Track(screen) => Mode::Synth(SynthScreen::default()),
    //     //                 _ => Mode::Error("Invalid mode".to_string()),
    //     //             };
    //     //         }
    //     //         Input::KeyUp(key) => {
    //     //             println!("Key up: {:?}", key);
    //     //         }
    //     //         Input::IncrementEncoder(encoder) => {
    //     //             println!("Increment encoder: {:?}", encoder);
    //     //         }
    //     //         Input::DecrementEncoder(encoder) => {
    //     //             println!("Decrement encoder: {:?}", encoder);
    //     //         }
    //     //     }
    //     // }
    // }
    // /// This function clears the buffer, draws to the buffer, and then fills the display with the buffer.
    // /// This lets us draw everything at once to prevent flickering.
    // pub fn draw<'a, D>(&mut self, display: &'a mut D) -> Result<(), Box<dyn std::error::Error>>
    // where
    //     D: DrawTarget,
    //     D::Color: RgbColor + From<RawU16>,
    // {
    //     // Clear the buffer
    //     self.buffer.clear(Rgb565::BLACK).unwrap();

    //     match &self.mode {
    //         Mode::Startup(screen) => screen.draw(&mut self.buffer)?,
    //         Mode::Track(screen) => screen.draw(&mut self.buffer)?,
    //         Mode::Synth(screen) => screen.draw(&mut self.buffer)?,
    //         Mode::Error(error) => {
    //             let style = MonoTextStyleBuilder::new()
    //                 .font(&mono_font::ascii::FONT_6X10)
    //                 .text_color(Rgb565::WHITE)
    //                 .background_color(Rgb565::RED)
    //                 .build();

    //             match Text::new(error, Point::new(6, 16), style).draw(&mut self.buffer) {
    //                 Ok(_) => {}
    //                 Err(_) => {}
    //             }
    //         }
    //     };

    //     let style = MonoTextStyleBuilder::new()
    //         .font(&mono_font::ascii::FONT_6X10)
    //         .text_color(Rgb565::BLACK)
    //         .background_color(Rgb565::WHITE)
    //         .build();

    //     match Text::new(&format!("FPS: {}", self.fps), Point::new(0, 10), style)
    //         .draw(&mut self.buffer)
    //     {
    //         Ok(_) => {}
    //         Err(_) => {}
    //     }

    //     // Convert the buffer to an iterator of colors
    //     // let colors =
    //     //     self.buffer.data().chunks_exact(2).map(|chunk| {
    //     //         D::Color::from(RawU16::from(u16::from_le_bytes([chunk[0], chunk[1]])))
    //     //     });

    //     // // Fill the display with the colors from the buffer
    //     // match display.fill_contiguous(&self.bounding_box, colors) {
    //     //     Ok(_) => {}
    //     //     Err(_) => {
    //     //         log::error!("Error filling display");
    //     //         return Err("Error filling display".into());
    //     //     }
    //     // }
    //     self.fps = self.fps_counter.tick();
    //     println!("FPS: {}", self.fps);
    //     Ok(())
    // }

    pub async fn run(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        // Start a display loop in a separate thread
        let display_handle = std::thread::spawn(|| {});

        // Start an input loop in a separate thread, possibly use a || async block
        loop {
            if let Some(inputs) = self.input.poll_input() {
                let ref mut mode = self.mode;
                let event = match mode {
                    Mode::Startup(screen) => screen.update(inputs),
                    Mode::Synth(screen) => screen.update(inputs),
                    Mode::Track(screen) => screen.update(inputs),
                    Mode::Error(screen) => None,
                };
                if let Some(event) = event {
                    self.next(event);
                }
            }
        }
    }

    fn handle_entry(&mut self) {
        match &mut self.mode {
            Mode::Startup(screen) => screen.entry(),
            Mode::Synth(screen) => screen.entry(),
            Mode::Track(screen) => screen.entry(),
            Mode::Error(screen) => {}
        }
    }

    fn handle_exit(&mut self) {
        match &mut self.mode {
            Mode::Startup(screen) => screen.exit(),
            Mode::Synth(screen) => screen.exit(),
            Mode::Track(screen) => screen.exit(),
            Mode::Error(screen) => {}
        }
    }
}

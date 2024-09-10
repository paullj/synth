use std::time::Instant;

use embedded_graphics::{
    framebuffer::{buffer_size, Framebuffer},
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::{
        raw::{LittleEndian, RawU16},
        Rgb565,
    },
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};

pub struct App {
    // TODO: Make this generic over the color type, and buffer size
    buffer:
        Framebuffer<Rgb565, RawU16, LittleEndian, 320, 240, { buffer_size::<Rgb565>(320, 240) }>,

    bounding_box: Rectangle,
}

impl App {
    pub fn new<'a, D>(display: &'a mut D) -> Self
    where
        D: DrawTarget,
        D::Color: RgbColor + From<RawU16>,
    {
        let _ = display.clear(D::Color::BLACK);

        let bounding_box = display.bounding_box();
        let buffer = Framebuffer::<
            Rgb565,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<Rgb565>(320, 240) },
        >::new();

        Self {
            buffer,
            bounding_box,
        }
    }

    /// This function clears the buffer, draws to the buffer, and then fills the display with the buffer.
    /// This lets us draw everything at once to prevent flickering.
    pub fn draw<'a, D>(&mut self, display: &'a mut D) -> Result<(), Box<dyn std::error::Error>>
    where
        D: DrawTarget,
        D::Color: RgbColor + From<RawU16>,
    {
        // Clear the buffer
        self.buffer.clear(Rgb565::BLACK).unwrap();

        // Draw everything to the buffer

        Circle::new(Point::new(100, 100), 30)
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
            .draw(&mut self.buffer)?;

        let style = MonoTextStyle::new(&FONT_9X18, Rgb565::WHITE);

        Text::new("Hello, world!", Point::new(10, 10), style).draw(&mut self.buffer)?;

        // Convert the buffer to an iterator of colors
        let colors =
            self.buffer.data().chunks_exact(2).map(|chunk| {
                D::Color::from(RawU16::from(u16::from_le_bytes([chunk[0], chunk[1]])))
            });
        // Fill the display with the colors from the buffer
        display
            .fill_contiguous(&self.bounding_box, colors)
            .map_err(|_| "Error drawing to display")?;

        Ok(())
    }
}

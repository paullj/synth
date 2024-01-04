pub mod display {
    pub const WIDTH: u32 = 65;
    pub const HEIGHT: u32 = 65;
    pub const FPS: u32 = 60;
}

pub mod color {
    use embedded_graphics::pixelcolor::{Rgb565, RgbColor};

    pub const BLACK: Rgb565 = Rgb565::BLACK;
    pub const WHITE: Rgb565 = Rgb565::WHITE;
    // pub const RED: Rgb565 = Rgb565::new(255, 0, 0);
}

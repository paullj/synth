pub(crate) mod color {
    use embedded_graphics::pixelcolor::{Rgb565, RgbColor};

    pub(crate) const WHITE: Rgb565 = Rgb565::WHITE;
    pub(crate) const BLACK: Rgb565 = Rgb565::BLACK;

    pub(crate) const ACCENT_A: Rgb565 = Rgb565::new(187, 133, 136);
    pub(crate) const ACCENT_B: Rgb565 = Rgb565::new(163, 163, 128);
    pub(crate) const ACCENT_C: Rgb565 = Rgb565::new(214, 206, 147);
    pub(crate) const ACCENT_D: Rgb565 = Rgb565::new(216, 164, 143);
}

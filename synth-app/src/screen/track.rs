use crate::screen::Screen;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{raw::RawU16, RgbColor},
};
use std::{convert::Infallible, sync::Arc};

#[derive(Debug, PartialEq)]
pub(crate) struct TrackScreen {
    selected: u8,
}

impl Default for TrackScreen {
    fn default() -> Self {
        Self { selected: 0 }
    }
}

impl Screen for TrackScreen {
    fn draw<'a, D>(&self, target: &'a mut D) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor + From<RawU16>,
    {
        target.clear(D::Color::BLACK);
        Ok(())
    }

    fn entry(&mut self) {
        todo!()
    }

    fn update(&mut self, inputs: Vec<crate::input::InputEvent>) -> Option<crate::mode::Event> {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}

use std::{convert::Infallible, sync::Arc};

use crossbeam::queue::SegQueue;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::RgbColor,
    prelude::*,
    text::Text,
};

use crate::app::{ActionMessage, State};
use super::{Event, Screen};

#[derive(Debug, PartialEq)]
pub(crate) struct ErrorScreen {
    pub(crate) message: String,
}

impl Screen for ErrorScreen {
    fn draw<D>(&self, target: &mut D, state: &State) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        match target.clear(D::Color::BLACK) {
            Ok(_) => {}
            Err(_) => panic!("Error clearing display"),
        }
        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, D::Color::WHITE);

        // Create a text at position (20, 30) and draw it using the previously defined style
        let text = Text::new(&self.message, Point::new(6, 16), style).draw(target);
        match text {
            Ok(_) => {}
            Err(_) => panic!("Error drawing text"),
        }
        Ok(())
    }

    fn update(&mut self, state: &State, _: Arc<SegQueue<ActionMessage>>) -> Option<Event> {
        None
    }

    fn entry(&mut self) {}

    fn exit(&mut self) {}
}

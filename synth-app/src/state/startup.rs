use std::{convert::Infallible, sync::Arc};

use crossbeam::queue::SegQueue;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::RgbColor,
    prelude::*,
    text::{Alignment, Text},
};

use crate::app::ActionMessage;

use super::{Event, Screen};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StartupScreen {}

impl Screen for StartupScreen {
    fn entry(&mut self) {}

    fn exit(&mut self) {}
    fn draw<D>(&self, target: &mut D, time: f64, delta: f64) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, D::Color::WHITE);

        target.clear(D::Color::BLACK);

        let text = Text::with_alignment(
            &format!("time={:.2}, ", time),
            Point::new(320 / 2, 160 / 2),
            style,
            Alignment::Center,
        )
        .draw(target);
        match text {
            Ok(_) => {}
            Err(_) => panic!("Error drawing text"),
        };

        Ok(())
    }

    fn update(
        &mut self,
        messages: Arc<SegQueue<ActionMessage>>,
        time: f64,
        delta: f64,
    ) -> Option<Event> {
        None
    }
}

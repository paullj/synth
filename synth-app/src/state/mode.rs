use std::{convert::Infallible, fmt, sync::Arc};

use crossbeam::queue::SegQueue;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{
        ascii::{FONT_10X20, FONT_6X10},
        MonoTextStyle,
    },
    pixelcolor::RgbColor,
    prelude::*,
    text::{Alignment, Text},
};

use crate::app::ActionMessage;

use super::{Event, Screen};

#[derive(Debug, PartialEq)]
pub(crate) enum Mode {
    Play,
    Compose,
    Edit,
}

impl Mode {
    fn next(&self) -> Self {
        use Mode::*;
        match *self {
            Play => Compose,
            Compose => Edit,
            Edit => Play,
        }
    }
    fn peek_next(&self) -> Self {
        use Mode::*;
        match self {
            Play => Compose,
            Compose => Edit,
            Edit => Play,
        }
    }
    fn peek_prev(&self) -> Self {
        use Mode::*;
        match self {
            Play => Edit,
            Compose => Play,
            Edit => Compose,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ModeScreen {
    pub(crate) selected_mode: Mode,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Play => write!(f, "Play"),
            Mode::Compose => write!(f, "Compose"),
            Mode::Edit => write!(f, "Edit"),
        }
    }
}

impl Screen for ModeScreen {
    fn entry(&mut self) {}

    fn exit(&mut self) {}
    fn draw<D>(&self, target: &mut D, time: f64, delta: f64) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        target.clear(D::Color::BLACK);

        // Create a new character style
        let style = MonoTextStyle::new(&FONT_10X20, D::Color::BLUE);
        let dim_style = MonoTextStyle::new(&FONT_6X10, D::Color::WHITE);

        let text = Text::with_alignment(
            &format!("{}", self.selected_mode),
            Point::new(320 / 2, 160 / 2),
            style,
            Alignment::Center,
        )
        .draw(target);

        let text = Text::with_alignment(
            &format!("{}", self.selected_mode.peek_prev()),
            Point::new(320 / 2, 160 / 2 + 20),
            dim_style,
            Alignment::Center,
        )
        .draw(target);

        let text = Text::with_alignment(
            &format!("{}", self.selected_mode.peek_next()),
            Point::new(320 / 2, 160 / 2 - 20),
            dim_style,
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
        while !messages.is_empty() {
            if let Some(action) = messages.pop() {
                match action {
                    ActionMessage::X => return Some(Event::CloseModeMenu),
                    ActionMessage::Y => self.selected_mode = self.selected_mode.next(),
                    _ => (),
                }
            }
        }
        None
    }
}

use std::{
    convert::Infallible,
    sync::Arc,
    time::{Duration, Instant},
};

use crossbeam::queue::SegQueue;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::RgbColor,
    prelude::*,
    text::{Alignment, Text},
};

use crate::app::{ActionMessage, State};

use super::{Event, Screen};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StartupScreen {
    pub(crate) time_entry: Option<Instant>,
}

const DURATION: f64 = 1.0;

impl Default for StartupScreen {
    fn default() -> Self {
        Self { time_entry: None }
    }
}

impl Screen for StartupScreen {
    fn entry(&mut self) {
        self.time_entry = Some(Instant::now());
    }

    fn exit(&mut self) {
        self.time_entry = None;
    }

    fn draw<D>(&self, target: &mut D, state: &State) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, D::Color::WHITE);

        target.clear(D::Color::BLACK);

        if let Some(_) = self.time_entry {
            let text = Text::with_alignment(
                "booting...",
                Point::new(320 / 2, 160 / 2),
                style,
                Alignment::Center,
            )
            .draw(target);
            match text {
                Ok(_) => {}
                Err(_) => panic!("Error drawing text"),
            };
        }

        Ok(())
    }

    fn update(&mut self, state: &State, _: Arc<SegQueue<ActionMessage>>) -> Option<Event> {
        if let Some(time) = self.time_entry {
            if (Instant::now() - time).as_secs_f64() > DURATION {
                return Some(Event::Initialized);
            }
        }

        None
    }
}

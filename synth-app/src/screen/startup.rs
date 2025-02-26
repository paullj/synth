use crate::{input::InputEvent, screen::Screen};

use embedded_graphics::{
    mono_font::{self, MonoTextStyleBuilder},
    pixelcolor::{raw::RawU16, Rgb555, Rgb565, Rgb888},
    prelude::*,
    text::Text,
};
use std::{convert::Infallible, time::Instant};

#[derive(Debug, PartialEq)]
pub(crate) struct StartupScreen {
    start_time: Option<Instant>,
}

impl Default for StartupScreen {
    fn default() -> Self {
        Self { start_time: None }
    }
}

const STARTUP_SCREEN_TIME: f64 = 2.0;

impl Screen for StartupScreen {
    fn draw<D>(&self, target: &mut D) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color:
            RgbColor + From<RawU16> + From<Rgb555> + From<Rgb565> + From<Rgb888> + Into<Rgb888>,
    {
        match target.clear(D::Color::BLACK) {
            Ok(_) => {}
            Err(_) => {}
        }

        let style = MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_10X20)
            .text_color(D::Color::WHITE)
            .background_color(D::Color::BLUE)
            .build();

        match Text::new(
            "PLACEHOLDER STARTUP SCREEN".into(),
            Point::new(6, 16),
            style,
        )
        .draw(target)
        {
            Ok(_) => {}
            Err(_) => {}
        }

        Ok(())
    }

    fn entry(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn update(&mut self, _inputs: Vec<InputEvent>) -> Option<crate::mode::Event> {
        if let Some(time) = self.start_time {
            if (Instant::now() - time).as_secs_f64() >= STARTUP_SCREEN_TIME {
                return Some(crate::mode::Event::Initialized);
            }
        }
        None
    }

    fn exit(&mut self) {
        self.start_time = None;
    }
}

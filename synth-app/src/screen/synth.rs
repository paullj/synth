use crate::{mode::Event, screen::Screen};
use embedded_graphics::{
    mono_font::{self, MonoTextStyleBuilder},
    pixelcolor::{raw::RawU16, Rgb555, Rgb565, Rgb888},
    prelude::*,
    text::Text,
};
use std::{convert::Infallible, sync::Arc};

#[derive(Debug, PartialEq)]
pub(crate) struct SynthScreen {
    encoder_value: i32,
}

impl Default for SynthScreen {
    fn default() -> Self {
        Self { encoder_value: 0 }
    }
}

impl Screen for SynthScreen {
    fn draw<'a, D>(&self, target: &'a mut D) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor + From<RawU16>,
    {
        target.clear(D::Color::BLACK);

        let style = MonoTextStyleBuilder::new()
            .font(&mono_font::ascii::FONT_10X20)
            .text_color(D::Color::WHITE)
            .build();

        match Text::new(
            format!("Encoder {}", self.encoder_value).as_str(),
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

    fn entry(&mut self) {}

    fn update(&mut self, inputs: Vec<crate::input::InputEvent>) -> Option<Event> {
        for input in inputs {
            match input {
                crate::input::InputEvent::IncrementEncoder(_) => {
                    self.encoder_value += 1;
                }
                crate::input::InputEvent::DecrementEncoder(_) => {
                    self.encoder_value -= 1;
                }
                _ => {}
            }
        }
        None
    }

    fn exit(&mut self) {}
}

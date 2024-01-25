use std::{convert::Infallible, sync::Arc};

use crossbeam::queue::SegQueue;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::RgbColor, prelude::*};

use crate::{app::ActionMessage, midi::bytes_to_midi};

use super::{Event, Screen};

#[derive(Debug, PartialEq)]
pub(crate) struct PlayScreen {}

const MENU: &[u8] = &[0x01, 0x01, 0x01];

impl Screen for PlayScreen {
    fn entry(&mut self) {}

    fn exit(&mut self) {}
    fn draw<D>(&self, target: &mut D, time: f64, delta: f64) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        target.clear(D::Color::BLACK);
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
                    ActionMessage::X => return Some(Event::OpenModeMenu),
                    _ => (),
                };
            }
        }
        None
    }
}

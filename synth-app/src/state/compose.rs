use std::{convert::Infallible, sync::Arc};

use crossbeam::queue::SegQueue;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::RgbColor, prelude::*};

use crate::app::{ActionMessage, State};

use super::{Event, Screen};

#[derive(Debug, PartialEq)]
pub(crate) struct ComposeScreen {}

impl Screen for ComposeScreen {
    fn entry(&mut self) {}

    fn exit(&mut self) {}
    fn draw<D>(&self, target: &mut D, state: &State) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        target.clear(D::Color::BLACK);
        Ok(())
    }

    fn update(&mut self, state: &State, actions: Arc<SegQueue<ActionMessage>>) -> Option<Event> {
        while !actions.is_empty() {
            if let Some(action) = actions.pop() {
                match action {
                    ActionMessage::X => return Some(Event::OpenModeMenu),
                    _ => (),
                };
            }
        }
        None
    }
}

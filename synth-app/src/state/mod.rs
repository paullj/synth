pub mod compose;
pub mod edit;
pub mod error;
pub mod mode;
pub mod play;
pub mod startup;

use crate::app::ActionMessage;

use self::{
    compose::ComposeScreen, edit::EditScreen, error::ErrorScreen, mode::ModeScreen,
    play::PlayScreen, startup::StartupScreen,
};
use crossbeam::queue::SegQueue;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::RgbColor};
use std::{convert::Infallible, fmt, sync::Arc};

#[derive(Debug)]
pub(crate) enum Event {
    Initialized,
    OpenModeMenu,
    CloseModeMenu,
    Error(String),
    Quit,
}

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Startup(StartupScreen),
    Play(PlayScreen),
    Mode(ModeScreen),
    Compose(ComposeScreen),
    Edit(EditScreen),
    Error(ErrorScreen),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Startup(_) => write!(f, "Startup"),
            State::Mode(_) => write!(f, "Mode"),
            State::Compose(_) => write!(f, "Compose"),
            State::Edit(_) => write!(f, "Edit"),
            State::Play(_) => write!(f, "Play"),
            State::Error(ErrorScreen { message }) => write!(f, "Error: {}", message),
        }
    }
}

pub(crate) trait Screen {
    fn entry(&mut self);
    fn update(
        &mut self,
        messages: Arc<SegQueue<ActionMessage>>,
        time: f64,
        delta: f64,
    ) -> Option<Event>;
    fn exit(&mut self);

    fn draw<D>(&self, target: &mut D, time: f64, delta: f64) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor;
}

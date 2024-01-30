pub mod compose;
pub mod edit;
pub mod error;
pub mod mode;
pub mod play;
pub mod startup;

use crate::app::{ActionMessage, State};

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
pub(crate) enum Machine {
    Startup(StartupScreen),
    Play(PlayScreen),
    Mode(ModeScreen),
    Compose(ComposeScreen),
    Edit(EditScreen),
    Error(ErrorScreen),
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Machine::Startup(_) => write!(f, "Startup"),
            Machine::Mode(_) => write!(f, "Mode"),
            Machine::Compose(_) => write!(f, "Compose"),
            Machine::Edit(_) => write!(f, "Edit"),
            Machine::Play(_) => write!(f, "Play"),
            Machine::Error(ErrorScreen { message }) => write!(f, "Error: {}", message),
        }
    }
}

pub(crate) trait Screen {
    fn entry(&mut self);
    fn update(&mut self, shared: &State, actions: Arc<SegQueue<ActionMessage>>) -> Option<Event>;
    fn exit(&mut self);
    fn draw<D>(&self, target: &mut D, shared: &State) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor;
}

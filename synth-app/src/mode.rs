use crate::screen::startup::StartupScreen;
use crate::screen::synth::SynthScreen;
use crate::screen::track::TrackScreen;

#[derive(Debug, PartialEq)]
pub(crate) enum Mode {
    Startup(StartupScreen),
    Track(TrackScreen),
    Synth(SynthScreen),
    Error(String),
}

pub(crate) enum Event {
    Initialized,
    Error(String),
    Quit,
}

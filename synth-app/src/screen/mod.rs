use std::convert::Infallible;

use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb555, Rgb565, Rgb888},
    prelude::*,
};

use crate::{input::InputEvent, mode::Event};

pub(crate) mod startup;
pub(crate) mod synth;
pub(crate) mod track;

pub(crate) trait Screen {
    fn entry(&mut self);

    fn update(&mut self, inputs: Vec<InputEvent>) -> Option<Event>;

    fn exit(&mut self);

    fn draw<'a, D>(&self, target: &'a mut D) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color:
            RgbColor + From<RawU16> + From<Rgb555> + From<Rgb565> + From<Rgb888> + Into<Rgb888>;
}

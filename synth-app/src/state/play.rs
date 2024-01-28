use easer::functions::{Cubic, Easing, Linear};
use std::{convert::Infallible, fmt, sync::Arc};

use crossbeam::queue::SegQueue;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::RgbColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Alignment, Text},
};

use crate::app::{ActionMessage, State};

use super::{Event, Screen};

#[derive(Debug, PartialEq)]
pub(crate) struct PlayScreen {
    pub(crate) selected_menu: EngineMenu,
}

#[derive(Debug, PartialEq)]
pub(crate) enum EngineMenu {
    Control = 0,
    ADSR = 1,
    Filter = 2,
    Effects = 3,
}

const MARGIN: i32 = 10;

impl EngineMenu {
    fn next(&self) -> Self {
        use EngineMenu::*;
        match *self {
            Control => ADSR,
            ADSR => Filter,
            Filter => Effects,
            Effects => Control,
        }
    }
}

impl fmt::Display for EngineMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineMenu::Control => write!(f, "Control"),
            EngineMenu::ADSR => write!(f, "ADSR"),
            EngineMenu::Filter => write!(f, "Filter"),
            EngineMenu::Effects => write!(f, "Effects"),
        }
    }
}

impl Default for PlayScreen {
    fn default() -> Self {
        Self {
            selected_menu: EngineMenu::Control,
        }
    }
}

impl Screen for PlayScreen {
    fn entry(&mut self) {}

    fn exit(&mut self) {}

    fn draw<D>(&self, target: &mut D, shared: &State) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        target.clear(D::Color::BLACK);
        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, D::Color::BLUE);

        let text = Text::with_alignment(
            &format!("{}", self.selected_menu),
            Point::new(320 / 2, 160 / 2),
            style,
            Alignment::Center,
        )
        .draw(target);

        match self.selected_menu {
            EngineMenu::Control => {}
            EngineMenu::ADSR => {
                let attack_start = Point {
                    x: MARGIN,
                    y: 160 - MARGIN,
                };
                let attack_end = Point {
                    x: attack_start.x
                        + Cubic::ease_out(shared.attack.value(), 0.0, 300.0 / 4.0, 5.0).round()
                            as i32,
                    y: MARGIN,
                };
                let decay_end = Point {
                    x: attack_end.x
                        + Cubic::ease_out(shared.decay.value(), 0.0, 300.0 / 4.0, 5.0).round()
                            as i32,
                    y: attack_end.y
                        + Linear::ease_out(1.0 - shared.sustain.value(), 0.0, 140.0, 1.0).round()
                            as i32,
                };
                let sustain_end = Point {
                    x: decay_end.x + 300 / 4,
                    y: decay_end.y,
                };
                let release_end = Point {
                    x: sustain_end.x
                        + Cubic::ease_out(shared.release.value(), 0.0, 300.0 / 4.0, 5.0).round()
                            as i32,
                    y: 160 - MARGIN,
                };
                Line::new(attack_start, attack_end)
                    .into_styled(PrimitiveStyle::with_stroke(D::Color::BLUE, 1))
                    .draw(target);
                Line::new(attack_end, decay_end)
                    .into_styled(PrimitiveStyle::with_stroke(D::Color::RED, 1))
                    .draw(target);
                Line::new(decay_end, sustain_end)
                    .into_styled(PrimitiveStyle::with_stroke(D::Color::GREEN, 1))
                    .draw(target);
                Line::new(sustain_end, release_end)
                    .into_styled(PrimitiveStyle::with_stroke(D::Color::YELLOW, 1))
                    .draw(target);
            }
            EngineMenu::Filter => {}
            EngineMenu::Effects => {}
        }

        match text {
            Ok(_) => {}
            Err(_) => panic!("Error drawing text"),
        };
        Ok(())
    }

    fn update(&mut self, shared: &State, actions: Arc<SegQueue<ActionMessage>>) -> Option<Event> {
        while !actions.is_empty() {
            if let Some(action) = actions.pop() {
                match action {
                    ActionMessage::X => return Some(Event::OpenModeMenu),
                    ActionMessage::Y => self.selected_menu = self.selected_menu.next(),
                    _ => (),
                };
            }
        }
        None
    }
}

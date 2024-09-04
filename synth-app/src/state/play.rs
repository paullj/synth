use easer::functions::{Cubic, Easing, Linear};
use std::{convert::Infallible, fmt, sync::Arc};

use crossbeam::queue::SegQueue;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::RgbColor,
    prelude::*,
    primitives::{Line, Polyline, PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};

use super::{Event, Screen};
use crate::app::{ActionMessage, State};

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

const MARGIN: i32 = 40;

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

fn bezier_curve<'a>(
    start: Point,
    control1: Point,
    control2: Point,
    end: Point,
    steps: u32,
) -> Polyline<'a> {
    let mut points = Vec::with_capacity(steps as usize);
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let x = (1.0 - t).powi(3) * start.x as f32
            + 3.0 * (1.0 - t).powi(2) * t * control1.x as f32
            + 3.0 * (1.0 - t) * t.powi(2) * control2.x as f32
            + t.powi(3) * end.x as f32;
        let y = (1.0 - t).powi(3) * start.y as f32
            + 3.0 * (1.0 - t).powi(2) * t * control1.y as f32
            + 3.0 * (1.0 - t) * t.powi(2) * control2.y as f32
            + t.powi(3) * end.y as f32;
        points.push(Point::new(x.round() as i32, y.round() as i32));
    }
    Polyline::new(Box::leak(points.into_boxed_slice()))
}

impl Screen for PlayScreen {
    fn entry(&mut self) {}
    fn exit(&mut self) {}
    fn draw<D>(&self, target: &mut D, shared: &State) -> Result<(), Infallible>
    where
        D: DrawTarget,
        D::Color: RgbColor,
    {
        let _ = target.clear(D::Color::BLACK);
        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, D::Color::BLUE);

        let text = Text::with_alignment(
            &format!("{}", self.selected_menu),
            Point::new(320 / 2, 240 / 2),
            style,
            Alignment::Center,
        )
        .draw(target);

        match self.selected_menu {
            EngineMenu::Control => {}
            EngineMenu::ADSR => {
                let attack_start = Point {
                    x: MARGIN,
                    y: 240 - MARGIN,
                };
                let attack_end = Point {
                    x: attack_start.x
                        + Cubic::ease_out(
                            shared.attack.value(),
                            0.0,
                            (320.0 - f64::from(MARGIN) * 2.0) / 4.0,
                            5.0,
                        )
                        .round() as i32,
                    y: MARGIN,
                };
                let attack_control_1 = attack_start
                    + Point {
                        x: 0,
                        y: (attack_end.y - attack_start.y) * 3 / 4,
                    };
                let attack_control_2 = attack_end
                    + Point {
                        x: -(attack_end.x - attack_start.x) * 3 / 4,
                        y: 0,
                    };
                let decay_end = Point {
                    x: attack_end.x
                        + Cubic::ease_out(
                            shared.decay.value(),
                            0.0,
                            (320.0 - f64::from(MARGIN) * 2.0) / 4.0,
                            5.0,
                        )
                        .round() as i32,
                    y: attack_end.y
                        + Linear::ease_out(
                            1.0 - shared.sustain.value(),
                            0.0,
                            (240.0 - f64::from(MARGIN) * 2.0),
                            1.0,
                        )
                        .round() as i32,
                };
                let decay_control_1 = attack_end
                    + Point {
                        x: 0,
                        y: (decay_end.y - attack_end.y) * 3 / 4,
                    };
                let decay_control_2 = decay_end
                    + Point {
                        x: -(decay_end.x - attack_end.x) * 3 / 4,
                        y: 0,
                    };
                let sustain_end = Point {
                    x: decay_end.x + 300 / 4,
                    y: decay_end.y,
                };

                let release_end = Point {
                    x: sustain_end.x
                        + Cubic::ease_out(
                            shared.release.value(),
                            0.0,
                            (320.0 - f64::from(MARGIN) * 2.0) / 4.0,
                            5.0,
                        )
                        .round() as i32,
                    y: 240 - MARGIN,
                };
                let release_control_1 = sustain_end
                    + Point {
                        x: 0,
                        y: (release_end.y - sustain_end.y) * 3 / 4,
                    };
                let release_control_2 = release_end
                    + Point {
                        x: -(release_end.x - sustain_end.x) * 3 / 4,
                        y: 0,
                    };

                let _attack_curve = bezier_curve(
                    attack_start,
                    attack_control_1,
                    attack_control_2,
                    attack_end,
                    50,
                )
                .into_styled(PrimitiveStyle::with_stroke(D::Color::BLUE, 1))
                .draw(target);

                let _decay_curve =
                    bezier_curve(attack_end, decay_control_1, decay_control_2, decay_end, 50)
                        .into_styled(PrimitiveStyle::with_stroke(D::Color::RED, 1))
                        .draw(target);

                let _ = Line::new(decay_end, sustain_end)
                    .into_styled(PrimitiveStyle::with_stroke(D::Color::GREEN, 1))
                    .draw(target);

                let _release_curve = bezier_curve(
                    sustain_end,
                    release_control_1,
                    release_control_2,
                    release_end,
                    50,
                )
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

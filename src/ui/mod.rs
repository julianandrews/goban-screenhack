mod annotations;
mod goban_display;
mod sgf_walker;

use crate::goban::{Goban, Stone, StoneColor};
use crate::sgf_parse::{SgfNode, SgfProp};
use annotations::{AnnotationDisplay, Annotations};
use goban_display::GobanDisplay;
use sgf_walker::{GameState, SgfWalker};
use std::error;
use std::time;

pub struct UI {
    goban: Goban,
    move_delay: u64,
    end_delay: u64,
    draw_annotations: bool,
    game_state: GameState,
    last_action_time: time::Instant,
    sgf_walker: SgfWalker,
    annotations: Annotations,
}

impl UI {
    const ANNOTATION_BOX_WIDTH: f32 = 0.3;

    pub fn new(
        sgfs: Vec<SgfNode>,
        move_delay: u64,
        end_delay: u64,
        draw_annotations: bool,
    ) -> Result<UI, Box<dyn error::Error>> {
        Ok(UI {
            goban: Goban::new((19, 19)),
            move_delay: move_delay,
            end_delay: end_delay,
            draw_annotations: draw_annotations,
            game_state: GameState::New,
            last_action_time: time::Instant::now(),
            sgf_walker: SgfWalker::new(sgfs)?,
            annotations: Annotations::new(),
        })
    }

    pub fn draw(&self, frame: &mut nanovg::Frame, width: f32, height: f32) {
        let aspect_ratio = if self.draw_annotations {
            1.0 + UI::ANNOTATION_BOX_WIDTH
        } else {
            1.0
        };
        let goban_size = if width > height * aspect_ratio {
            height
        } else {
            width / aspect_ratio
        };
        let transform = nanovg::Transform::new()
            .translate((width - goban_size * aspect_ratio) / 2.0, (height - goban_size) / 2.0);
        frame.transformed(transform, |mut frame| {
            GobanDisplay::new(&self.goban).draw(&mut frame, goban_size, goban_size)
        });
        if self.draw_annotations {
            frame.transformed(transform.translate(goban_size, 0.0), |mut frame| {
                AnnotationDisplay::new(&self.annotations).draw(
                    &mut frame,
                    goban_size * UI::ANNOTATION_BOX_WIDTH,
                    goban_size,
                )
            });
        }
    }

    pub fn update_game_state(&mut self) -> Result<(), Box<dyn error::Error>> {
        match self.game_state {
            GameState::New => {
                self.goban = Goban::new(self.get_board_size());
                self.game_state = GameState::Ongoing;
            }
            GameState::Ongoing => {
                if self.last_action_time.elapsed() > time::Duration::from_millis(self.move_delay) {
                    self.game_state = self.process_current_node()?;
                    self.last_action_time = std::time::Instant::now();
                }
            }
            GameState::Ended => {
                if self.last_action_time.elapsed() > time::Duration::from_millis(self.end_delay) {
                    self.game_state = GameState::New;
                    self.last_action_time = std::time::Instant::now();
                }
            }
        }

        Ok(())
    }

    fn process_current_node(&mut self) -> Result<GameState, Box<dyn error::Error>> {
        for prop in self.sgf_walker.node().properties() {
            match prop {
                SgfProp::B(sgf_parse::Move::Move(point)) => {
                    if !self.is_tt_pass(point) {
                        self.goban
                            .play_stone(Stone::new(point.x, point.y, StoneColor::Black))?;
                    }
                }
                SgfProp::W(sgf_parse::Move::Move(point)) => {
                    if !self.is_tt_pass(point) {
                        self.goban
                            .play_stone(Stone::new(point.x, point.y, StoneColor::White))?;
                    }
                }
                SgfProp::AB(points) => {
                    for point in points.iter() {
                        self.goban
                            .add_stone(Stone::new(point.x, point.y, StoneColor::Black))?;
                    }
                }
                SgfProp::AW(points) => {
                    for point in points.iter() {
                        self.goban
                            .add_stone(Stone::new(point.x, point.y, StoneColor::White))?;
                    }
                }
                SgfProp::AE(points) => {
                    for point in points.iter() {
                        self.goban.clear_point((point.x, point.y));
                    }
                }
                SgfProp::MN(num) => self.goban.set_move_number(*num as u64),
                _ => {}
            }
        }

        Ok(self.sgf_walker.next_node())
    }

    fn is_tt_pass(&self, point: &sgf_parse::Point) -> bool {
        point.x == 19 && point.y == 19 && self.goban.size.0 < 20 && self.goban.size.1 < 20
    }

    fn get_board_size(&self) -> (u8, u8) {
        match self.sgf_walker.node().get_property("SZ") {
            Some(SgfProp::SZ(size)) => size.clone(),
            None => (19, 19),
            _ => unreachable!(),
        }
    }
}

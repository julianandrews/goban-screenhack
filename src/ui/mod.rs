mod goban_display;
mod sgf_walker;

use crate::goban::{Goban, Stone, StoneColor};
use crate::sgf_parse::{go, SgfNode};
use goban_display::GobanDisplay;
use sgf_walker::{GameState, SgfWalker};
use std::error;
use std::time;

pub struct UI {
    goban: Goban,
    move_delay: u64,
    end_delay: u64,
    game_state: GameState,
    last_action_time: time::Instant,
    sgf_walker: SgfWalker,
}

impl UI {
    pub fn new(
        sgfs: Vec<SgfNode<go::Prop>>,
        move_delay: u64,
        end_delay: u64,
    ) -> Result<UI, Box<dyn error::Error>> {
        Ok(UI {
            goban: Goban::new((19, 19)),
            move_delay: move_delay,
            end_delay: end_delay,
            game_state: GameState::New,
            last_action_time: time::Instant::now(),
            sgf_walker: SgfWalker::new(sgfs)?,
        })
    }

    pub fn draw(&self, frame: &mut nanovg::Frame, width: f32, height: f32) {
        let goban_display = GobanDisplay::new(&self.goban);
        goban_display.draw(frame, width, height);
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
                go::Prop::B(go::Move::Move(point)) => {
                    if !self.is_tt_pass(point) {
                        self.goban
                            .play_stone(Stone::new(point.x, point.y, StoneColor::Black))?;
                    }
                }
                go::Prop::W(go::Move::Move(point)) => {
                    if !self.is_tt_pass(point) {
                        self.goban
                            .play_stone(Stone::new(point.x, point.y, StoneColor::White))?;
                    }
                }
                go::Prop::AB(points) => {
                    for point in points.iter() {
                        self.goban
                            .add_stone(Stone::new(point.x, point.y, StoneColor::Black))?;
                    }
                }
                go::Prop::AW(points) => {
                    for point in points.iter() {
                        self.goban
                            .add_stone(Stone::new(point.x, point.y, StoneColor::White))?;
                    }
                }
                go::Prop::AE(points) => {
                    for point in points.iter() {
                        self.goban.clear_point((point.x, point.y));
                    }
                }
                go::Prop::MN(num) => self.goban.set_move_number(*num as u64),
                _ => {}
            }
        }

        Ok(self.sgf_walker.next_node())
    }

    fn is_tt_pass(&self, point: &go::Point) -> bool {
        point.x == 19 && point.y == 19 && self.goban.size.0 < 20 && self.goban.size.1 < 20
    }

    fn get_board_size(&self) -> (u8, u8) {
        match self.sgf_walker.node().get_property("SZ") {
            Some(go::Prop::SZ(size)) => size.clone(),
            None => (19, 19),
            _ => unreachable!(),
        }
    }
}

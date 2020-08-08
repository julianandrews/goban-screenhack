use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time;

use super::goban;
use super::goban_display;
use super::sgf_parse::SgfProp;

pub struct UI {
    goban: goban::Goban,
    move_delay: u64,
    end_delay: u64,
    game_state: GameState,
    last_action_time: time::Instant,
    sgfs: Vec<sgf_parse::SgfNode>,
    sgf_node: sgf_parse::SgfNode,
}

impl UI {
    pub fn new(sgfs: Vec<sgf_parse::SgfNode>, move_delay: u64, end_delay: u64) -> Result<UI, UIError> {
        if sgfs.is_empty() {
            Err(UIError::NoSgfs)?;
        }
        let mut rng = thread_rng();
        let sgf_node = sgfs.choose(&mut rng).unwrap().clone(); // sgfs is never empty
        Ok(UI {
            goban: goban::Goban::new((19, 19)),
            game_state: GameState::New,
            last_action_time: time::Instant::now(),
            sgfs: sgfs,
            sgf_node: sgf_node,
            move_delay: move_delay,
            end_delay: end_delay,
        })
    }

    pub fn update_game_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.game_state {
            GameState::New => {
                let board_size = match self.sgf_node.get_property("SZ") {
                    Some(SgfProp::SZ(size)) => size.clone(),
                    None => (19, 19),
                    _ => unreachable!(),
                };
                self.reset(board_size);
                self.game_state = GameState::Ongoing;
            }
            GameState::Ongoing => {
                if self.last_action_time.elapsed() > time::Duration::from_millis(self.move_delay) {
                    // TODO: Fix this unecessary clone
                    let props: Vec<SgfProp> = self.sgf_node.properties().cloned().collect();
                    for prop in props {
                        match prop {
                            SgfProp::B(sgf_parse::Move::Move(point)) => {
                                if point.x == 19
                                    && point.y == 19
                                    && self.board_size().0 < 20
                                    && self.board_size().1 < 20
                                {
                                    continue; // "tt" pass
                                }
                                let stone = goban::Stone {
                                    x: point.x,
                                    y: point.y,
                                    color: goban::StoneColor::Black,
                                };
                                self.play_stone(stone)?;
                            }
                            SgfProp::W(sgf_parse::Move::Move(point)) => {
                                if point.x == 19
                                    && point.y == 19
                                    && self.board_size().0 < 20
                                    && self.board_size().1 < 20
                                {
                                    continue; // "tt" pass
                                }
                                let stone = goban::Stone {
                                    x: point.x,
                                    y: point.y,
                                    color: goban::StoneColor::White,
                                };
                                self.play_stone(stone)?;
                            }
                            SgfProp::AB(points) => {
                                for point in points.iter() {
                                    let stone = goban::Stone {
                                        x: point.x,
                                        y: point.y,
                                        color: goban::StoneColor::Black,
                                    };
                                    self.add_stone(stone)?;
                                }
                            }
                            SgfProp::AW(points) => {
                                for point in points.iter() {
                                    let stone = goban::Stone {
                                        x: point.x,
                                        y: point.y,
                                        color: goban::StoneColor::White,
                                    };
                                    self.add_stone(stone)?;
                                }
                            }
                            SgfProp::AE(points) => {
                                for point in points.iter() {
                                    self.clear_point((point.x, point.y));
                                }
                            }
                            SgfProp::MN(num) => self.set_move_number(num as u64),
                            _ => {}
                        }
                    }
                    // TODO: avoid these stupid clones
                    self.sgf_node = match self.sgf_node.clone().into_iter().next() {
                        None => {
                            self.game_state = GameState::Ended;
                            let mut rng = thread_rng();
                            self.sgfs.choose(&mut rng).unwrap().clone() // sgfs is never empty
                        }
                        Some(node) => node,
                    };
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

    pub fn draw(&self, frame: &mut nanovg::Frame, width: f32, height: f32) {
        let goban_display = goban_display::GobanDisplay::new(&self.goban);
        goban_display.draw(frame, width, height);
    }

    fn add_stone(&mut self, stone: goban::Stone) -> Result<(), goban::GobanError> {
        self.goban.add_stone(stone)
    }

    fn play_stone(&mut self, stone: goban::Stone) -> Result<(), goban::GobanError> {
        self.goban.play_stone(stone)
    }

    fn clear_point(&mut self, point: (u8, u8)) {
        self.goban.clear_point(point);
    }

    fn reset(&mut self, board_size: (u8, u8)) {
        self.goban = goban::Goban::new(board_size);
    }

    fn set_move_number(&mut self, num: u64) {
        self.goban.set_move_number(num);
    }

    fn board_size(&self) -> (u8, u8) {
        self.goban.size
    }
}

enum GameState {
    New,
    Ongoing,
    Ended,
}

#[derive(Debug)]
pub enum UIError {
    NoSgfs,
}

impl std::fmt::Display for UIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            UIError::NoSgfs => write!(f, "No valid sgf files found in search path."),
        }
    }
}

impl std::error::Error for UIError {}

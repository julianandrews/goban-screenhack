use std::collections::{HashMap, HashSet, VecDeque};

pub use super::sgf::{StoneColor, Stone};

pub struct Goban {
    pub size: BoardSize,
    pub stones: HashMap<(u8, u8), StoneColor>,
    pub move_number: u64,
    pub black_captures: u64,
    pub white_captures: u64,
}

impl Goban {
    pub fn new(board_size: BoardSize) -> Goban {
        Goban {
            size: board_size,
            stones: HashMap::new(),
            move_number: 0,
            black_captures: 0,
            white_captures: 0,
        }
    }

    pub fn stones(&self) -> impl Iterator<Item = Stone> {
        self.stones
            .iter()
            .map(|(point, color)| Stone {
                x: point.0,
                y: point.1,
                color: *color,
            })
            .collect::<Vec<Stone>>()
            .into_iter()
    }

    pub fn add_stone(&mut self, stone: Stone) -> Result<(), GobanError> {
        let key = (stone.x, stone.y);
        if self.stones.contains_key(&key) {
            Err(GobanError::InvalidMoveError)?;
        }
        self.stones.insert(key, stone.color);
        let opponent_color = match stone.color {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        };
        // Remove any neighboring groups with no liberties.
        for neighbor in self.neighbors(key) {
            if let Some(color) = self.stones.get(&neighbor) {
                if *color == opponent_color {
                    self.process_captures(&neighbor);
                }
            }
        }
        // Now remove the played stone if still neccessary
        self.process_captures(&key);

        Ok(())
    }

    pub fn play_stone(&mut self, stone: Stone) -> Result<(), GobanError> {
        self.add_stone(stone)?;
        self.move_number += 1;

        Ok(())
    }

    fn neighbors(&self, point: (u8, u8)) -> impl Iterator<Item = (u8, u8)> {
        let (x, y) = point;
        let neighbors = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        neighbors.into_iter()
    }

    fn process_captures(&mut self, start_point: &(u8, u8)) {
        let group_color = match self.stones.get(start_point) {
            Some(color) => color,
            None => return,
        };
        let mut group = HashSet::new();
        let mut to_process = VecDeque::new();
        to_process.push_back(start_point.clone());
        while let Some(p) = to_process.pop_back() {
            group.insert(p);
            for neighbor in self.neighbors(p) {
                if group.contains(&neighbor) {
                    continue;
                }
                match self.stones.get(&neighbor) {
                    None => return,
                    Some(c) if c == group_color => to_process.push_back(neighbor.clone()),
                    _ => {}
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum GobanError {
    InvalidMoveError,
}

#[derive(Copy, Clone)]
pub enum BoardSize {
    Nine = 9,
    Thirteen = 13,
    Nineteen = 19,
}

impl std::fmt::Display for GobanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GobanError::InvalidMoveError => write!(f, "Invalid move"),
        }
    }
}

impl std::error::Error for GobanError {}

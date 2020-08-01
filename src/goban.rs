use std::collections::HashMap;

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
        self.stones.iter().map(|(point, color)| Stone {
            x: point.0,
            y: point.1,
            color: *color,
        }).collect::<Vec<Stone>>().into_iter()
    }

    pub fn add_stone(&mut self, stone: Stone) -> Result<(), GobanError> {
        let key = (stone.x, stone.y);
        if self.stones.contains_key(&key) {
            Err(GobanError::InvalidMoveError)?;
        }
        self.stones.insert(key, stone.color);
        self.process_captures();

        Ok(())
    }

    pub fn play_stone(&mut self, stone: Stone) -> Result<(), GobanError> {
        self.add_stone(stone)?;
        self.move_number += 1;

        Ok(())
    }

    fn process_captures(&mut self) {
        // TODO
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

#[derive(Copy, Clone, Debug)]
pub enum StoneColor {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
pub struct Stone {
    pub x: u8,
    pub y: u8,
    pub color: StoneColor,
}

impl std::fmt::Display for GobanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GobanError::InvalidMoveError => write!(f, "Invalid move")
        }
    }
}

impl std::error::Error for GobanError {}


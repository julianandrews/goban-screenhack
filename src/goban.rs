use std::collections::{HashMap, HashSet, VecDeque};

pub struct Goban {
    pub size: (u8, u8),
    pub stones: HashMap<(u8, u8), StoneColor>,
    pub move_number: u64,
    pub black_captures: u64,
    pub white_captures: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl Goban {
    pub fn new(board_size: (u8, u8)) -> Goban {
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
        // TODO: Validate that the stone is in range for the board.
        let key = (stone.x, stone.y);
        if self.stones.contains_key(&key) {
            Err(GobanError::InvalidMoveError)?;
        }
        self.stones.insert(key, stone.color);

        Ok(())
    }

    pub fn play_stone(&mut self, stone: Stone) -> Result<(), GobanError> {
        self.add_stone(stone)?;
        let opponent_color = match stone.color {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        };
        // Remove any neighboring groups with no liberties.
        let key = (stone.x, stone.y);
        for neighbor in self.neighbors(key) {
            if let Some(color) = self.stones.get(&neighbor) {
                if *color == opponent_color {
                    self.process_captures(&neighbor);
                }
            }
        }
        // Now remove the played stone if still neccessary
        self.process_captures(&key);
        self.move_number += 1;

        Ok(())
    }

    pub fn clear_point(&mut self, point: (u8, u8)) {
        self.stones.remove(&point);
    }

    fn neighbors(&self, point: (u8, u8)) -> impl Iterator<Item = (u8, u8)> {
        let (x, y) = point;
        let mut neighbors = vec![];
        if x < self.size.0 - 1 {
            neighbors.push((x + 1, y));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y < self.size.1 - 1 {
            neighbors.push((x, y + 1));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }

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
                    Some(c) if c == group_color => {
                        to_process.push_back(neighbor.clone());
                    }
                    _ => {}
                }
            }
        }
        for stone in group {
            self.stones.remove(&stone);
        }
    }
}

#[derive(Debug)]
pub enum GobanError {
    InvalidMoveError,
}

impl std::fmt::Display for GobanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GobanError::InvalidMoveError => write!(f, "Invalid move"),
        }
    }
}

impl std::error::Error for GobanError {}

#[allow(dead_code)] // TODO!
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
    pub x: u32,
    pub y: u32,
    pub color: StoneColor,
}

pub struct Goban {
    pub size: BoardSize,
    stones: Vec<Stone>,
}

impl Goban {
    pub fn new() -> Goban {
        Goban {
            size: BoardSize::Nineteen,
            stones: vec![
                Stone { x: 9, y: 9, color: StoneColor::Black },
                Stone { x: 9, y: 8, color: StoneColor::Black },
                Stone { x: 8, y: 8, color: StoneColor::Black },
                Stone { x: 7, y: 9, color: StoneColor::White },
                Stone { x: 8, y: 9, color: StoneColor::White },
                Stone { x: 9, y: 10, color: StoneColor::White },
                Stone { x: 10, y: 10, color: StoneColor::White },
            ],
        }
    }

    pub fn stones(&self) -> impl Iterator<Item = &Stone> {
        self.stones.iter()
    }

    pub fn step(&mut self) {
        // TODO!
    }
}

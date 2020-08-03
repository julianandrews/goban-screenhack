use super::goban;
use super::goban_display;

pub struct UI {
    goban: goban::Goban,
}

impl UI {
    pub fn new() -> UI {
        UI {
            goban: goban::Goban::new(goban::BoardSize::Nineteen),
        }
    }

    pub fn draw(&self, frame: &mut nanovg::Frame, width: f32, height: f32) {
        let goban_display = goban_display::GobanDisplay::new(&self.goban);
        goban_display.draw(frame, width, height);
    }

    pub fn add_stone(&mut self, stone: goban::Stone) -> Result<(), goban::GobanError> {
        self.goban.add_stone(stone)
    }

    pub fn play_stone(&mut self, stone: goban::Stone) -> Result<(), goban::GobanError> {
        self.goban.play_stone(stone)
    }

    pub fn reset(&mut self, board_size: goban::BoardSize) {
        self.goban = goban::Goban::new(board_size);
    }
}

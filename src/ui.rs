use std::rc::Rc;

use super::goban;
use super::goban_display;

pub struct UI {
    goban_display: goban_display::GobanDisplay,
}

impl UI {
    pub fn new() -> UI {
        let goban = Rc::new(goban::Goban::new());
        let goban_display = goban_display::GobanDisplay::new(goban);

        UI {
            goban_display: goban_display,
        }
    }

    pub fn draw(&self, frame: &mut nanovg::Frame, width: f32, height: f32) {
        self.goban_display.draw(frame, width, height);
    }
}

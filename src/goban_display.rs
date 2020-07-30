use std::rc::Rc;

use super::goban;

pub struct GobanDisplay {
    goban: Rc<goban::Goban>,
}

impl GobanDisplay {
    // TODO: Line color, board color
    const LINE_WIDTH: f32 = 1.0 / 396.0;
    const BORDER_WIDTH: f32 = 1.0 / 198.0;
    const HOSHI_RADIUS: f32 = 1.0 / 198.0;
    const NINE_HOSHIS: [(u32, u32); 4] = [(2, 2), (2, 6), (6, 2), (6, 6)];
    const THIRTEEN_HOSHIS: [(u32, u32); 5] = [(3, 3), (3, 9), (6, 6), (9, 3), (9, 9)];
    const NINETEEN_HOSHIS: [(u32, u32); 9] = [
        (3, 3),
        (3, 9),
        (3, 15),
        (9, 3),
        (9, 9),
        (9, 15),
        (15, 3),
        (15, 9),
        (15, 15),
    ];

    pub fn new(goban: Rc<goban::Goban>) -> GobanDisplay {
        GobanDisplay { goban: goban }
    }

    fn draw_board_contents(&self, frame: &nanovg::Frame) {
        let line_color: nanovg::Color = nanovg::Color::new(0.0, 0.0, 0.0, 1.0);
        let border_options = nanovg::StrokeOptions {
            width: GobanDisplay::BORDER_WIDTH,
            ..Default::default()
        };
        let line_options = nanovg::StrokeOptions {
            width: GobanDisplay::LINE_WIDTH,
            ..Default::default()
        };
        frame.path(
            |path| {
                path.rect((0.0, 0.0), (1.0, 1.0));
                path.stroke(line_color, border_options);
            },
            Default::default(),
        );
        frame.path(
            |path| {
                for i in 1..self.goban.size as u32 - 1 {
                    path.move_to((i as f32 * self.line_spacing(), 0.0));
                    path.line_to((i as f32 * self.line_spacing(), 1.0));
                    path.move_to((0.0, i as f32 * self.line_spacing()));
                    path.line_to((1.0, i as f32 * self.line_spacing()));
                }
                path.stroke(line_color, line_options);
            },
            Default::default(),
        );

        for &(x, y) in self.hoshi_points() {
            frame.path(
                |path| {
                    path.circle((0.0, 0.0), GobanDisplay::HOSHI_RADIUS / self.line_spacing());
                    path.fill(line_color, Default::default());
                },
                nanovg::PathOptions {
                    transform: Some(self.point_frame_transform(x, y)),
                    ..Default::default()
                },
            );
        }

        for &stone in self.goban.stones() {
            self.draw_stone(&frame, stone);
        }
    }

    fn draw_stone(&self, frame: &nanovg::Frame, stone: goban::Stone) {
        frame.path(
            |path| {
                path.circle((0.025, 0.025), 0.475);
                path.fill(nanovg::Color::new(0.0, 0.0, 0.0, 0.5), Default::default());
            },
            nanovg::PathOptions {
                transform: Some(self.point_frame_transform(stone.x, stone.y)),
                ..Default::default()
            },
        );
        let paint = match stone.color {
            goban::StoneColor::Black => nanovg::Gradient::Radial {
                center: (-0.17, -0.2),
                inner_radius: 0.0,
                outer_radius: 1.0,
                start_color: nanovg::Color::new(0.25, 0.25, 0.25, 1.0),
                end_color: nanovg::Color::new(0.0, 0.0, 0.0, 1.0),
            },
            goban::StoneColor::White => nanovg::Gradient::Radial {
                center: (-0.17, -0.2),
                inner_radius: 0.0,
                outer_radius: 1.0,
                start_color: nanovg::Color::new(1.0, 1.0, 1.0, 1.0),
                end_color: nanovg::Color::new(0.6, 0.6, 0.6, 1.0),
            },
        };
        frame.path(
            |path| {
                path.circle((-0.017, -0.017), 0.475);
                path.fill(paint, Default::default());
            },
            nanovg::PathOptions {
                transform: Some(self.point_frame_transform(stone.x, stone.y)),
                ..Default::default()
            },
        );
    }

    fn draw_board(&self, mut frame: nanovg::Frame) {
        let board_color: nanovg::Color = nanovg::Color::new(0.9, 0.73, 0.37, 1.0);
        frame.path(
            |path| {
                path.rect((0.0, 0.0), (1.0, 1.0));
                path.fill(board_color, Default::default());
            },
            Default::default(),
        );

        let board_margin = 14.1 / 22.0 / (self.goban.size as u32 as f32 - 1.0);
        let scale = 1.0 - 2.0 * board_margin;
        let transform = nanovg::Transform::new()
            .with_translation(board_margin, board_margin)
            .with_scale(scale, scale);
        frame.transformed(transform, |frame| self.draw_board_contents(&frame));
    }

    pub fn draw(&self, frame: &mut nanovg::Frame, width: f32, height: f32) {
        let transform = if width > height {
            nanovg::Transform::new()
                .translate((width - height) / 2.0, 0.0)
                .scale(height, height)
        } else {
            nanovg::Transform::new()
                .translate(0.0, (height - width) / 2.0)
                .scale(width, width)
        };
        frame.transformed(transform, |frame| self.draw_board(frame));
    }

    fn point_frame_transform(&self, x: u32, y: u32) -> nanovg::Transform {
        let scale = self.line_spacing();
        nanovg::Transform::new()
            .with_translation(scale * x as f32, scale * y as f32)
            .with_scale(scale, scale)
    }

    fn hoshi_points(&self) -> impl Iterator<Item = &(u32, u32)> {
        match self.goban.size {
            goban::BoardSize::Nine => GobanDisplay::NINE_HOSHIS.iter(),
            goban::BoardSize::Thirteen => GobanDisplay::THIRTEEN_HOSHIS.iter(),
            goban::BoardSize::Nineteen => GobanDisplay::NINETEEN_HOSHIS.iter(),
        }
    }

    fn line_spacing(&self) -> f32 {
        1.0 / (self.goban.size as u32 - 1) as f32
    }
}

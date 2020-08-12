use super::{Goban, Stone, StoneColor};

pub struct GobanDisplay<'a> {
    goban: &'a Goban,
}

impl<'a> GobanDisplay<'a> {
    const BOARD_COLOR: (f32, f32, f32) = (0.9, 0.73, 0.37);
    const LINE_WIDTH: f32 = 1.0 / 22.0;
    const BORDER_WIDTH: f32 = 1.0 / 11.0;
    const BOARD_MARGIN: f32 = 14.1 / 22.0;
    const HOSHI_RADIUS: f32 = 1.0 / 11.0;
    const DEFAULT_HOSHIS: [(u8, u8); 0] = [];
    const NINE_HOSHIS: [(u8, u8); 4] = [(2, 2), (2, 6), (6, 2), (6, 6)];
    const THIRTEEN_HOSHIS: [(u8, u8); 5] = [(3, 3), (3, 9), (6, 6), (9, 3), (9, 9)];
    const NINETEEN_HOSHIS: [(u8, u8); 9] = [
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

    pub fn new(goban: &'a Goban) -> GobanDisplay<'a> {
        GobanDisplay { goban: goban }
    }

    pub fn draw(&self, frame: &mut nanovg::Frame, width: f32, height: f32) {
        // Transform to a frame where the spacing between lines is 1.0 and the origin is in the
        // upper right corner of the board drawing area.
        let board_width = (self.goban.size.0 - 1) as f32 + 2.0 * GobanDisplay::BOARD_MARGIN;
        let board_height = (self.goban.size.1 - 1) as f32 + 2.0 * GobanDisplay::BOARD_MARGIN;
        let aspect_ratio = board_width / board_height;
        let transform = if width > height * aspect_ratio {
            let scale = height / board_height;
            let offset = (width - height * aspect_ratio) / 2.0;
            nanovg::Transform::new()
                .translate(offset, 0.0)
                .scale(scale, scale)
        } else {
            let scale = width / board_width;
            let offset = (height - width / aspect_ratio) / 2.0;
            nanovg::Transform::new()
                .translate(0.0, offset)
                .scale(scale, scale)
        };
        // Draw the board itself.
        frame.path(
            |path| {
                let board_color = nanovg::Color::new(
                    GobanDisplay::BOARD_COLOR.0,
                    GobanDisplay::BOARD_COLOR.1,
                    GobanDisplay::BOARD_COLOR.2,
                    1.0,
                );
                path.rect((0.0, 0.0), (board_width, board_height));
                path.fill(board_color, Default::default());
            },
            nanovg::PathOptions {
                transform: Some(transform),
                ..Default::default()
            },
        );

        // Now move the origin to (0, 0), and draw the lines and stones.
        let transform = transform.translate(GobanDisplay::BOARD_MARGIN, GobanDisplay::BOARD_MARGIN);
        frame.transformed(transform, |frame| self.draw_board(&frame));
    }

    // Draw the board, assuming the spacing between lines is 1.0.
    fn draw_board(&self, frame: &nanovg::Frame) {
        let line_color: nanovg::Color = nanovg::Color::new(0.0, 0.0, 0.0, 1.0);
        let border_options = nanovg::StrokeOptions {
            width: GobanDisplay::BORDER_WIDTH,
            ..Default::default()
        };
        let line_options = nanovg::StrokeOptions {
            width: GobanDisplay::LINE_WIDTH,
            ..Default::default()
        };
        let width = (self.goban.size.0 - 1) as u32 as f32;
        let height = (self.goban.size.1 - 1) as u32 as f32;
        frame.path(
            |path| {
                path.rect((0.0, 0.0), (width, height));
                path.stroke(line_color, border_options);
            },
            Default::default(),
        );
        frame.path(
            |path| {
                for i in 1..self.goban.size.0 as u32 - 1 {
                    path.move_to((i as f32, 0.0));
                    path.line_to((i as f32, height));
                }
                for i in 1..self.goban.size.1 as u32 - 1 {
                    path.move_to((0.0, i as f32));
                    path.line_to((width, i as f32));
                }
                path.stroke(line_color, line_options);
            },
            Default::default(),
        );

        for &(x, y) in self.hoshi_points() {
            frame.path(
                |path| {
                    path.circle((0.0, 0.0), GobanDisplay::HOSHI_RADIUS);
                    path.fill(line_color, Default::default());
                },
                nanovg::PathOptions {
                    transform: Some(self.point_frame_transform(x, y)),
                    ..Default::default()
                },
            );
        }

        for stone in self.goban.stones() {
            self.draw_stone(&frame, stone);
        }
    }

    // Draw a stone centered at 0.0 assuming interline spacing of 1.0.
    fn draw_stone(&self, frame: &nanovg::Frame, stone: Stone) {
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
            StoneColor::Black => nanovg::Gradient::Radial {
                center: (-0.17, -0.2),
                inner_radius: 0.0,
                outer_radius: 1.0,
                start_color: nanovg::Color::new(0.25, 0.25, 0.25, 1.0),
                end_color: nanovg::Color::new(0.0, 0.0, 0.0, 1.0),
            },
            StoneColor::White => nanovg::Gradient::Radial {
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

    fn point_frame_transform(&self, x: u8, y: u8) -> nanovg::Transform {
        nanovg::Transform::new().with_translation(x as f32, y as f32)
    }

    fn hoshi_points(&self) -> impl Iterator<Item = &(u8, u8)> {
        match self.goban.size {
            (9, 9) => GobanDisplay::NINE_HOSHIS.iter(),
            (13, 13) => GobanDisplay::THIRTEEN_HOSHIS.iter(),
            (19, 19) => GobanDisplay::NINETEEN_HOSHIS.iter(),
            _ => GobanDisplay::DEFAULT_HOSHIS.iter(),
        }
    }
}

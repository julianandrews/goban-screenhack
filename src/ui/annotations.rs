pub struct Annotations {
    black: PlayerInfo,
    white: PlayerInfo,
    name: Option<String>,
    comment: Option<String>,
}

impl Annotations {
    pub fn new() -> Annotations {
        Annotations {
            black: PlayerInfo {
                name: None,
                rank: None,
            },
            white: PlayerInfo {
                name: None,
                rank: None,
            },
            name: None,
            comment: None,
        }
    }
}

pub struct AnnotationDisplay<'a> {
    annotations: &'a Annotations,
}

impl<'a> AnnotationDisplay<'a> {
    pub fn new(annotations: &'a Annotations) -> AnnotationDisplay {
        AnnotationDisplay {
            annotations: annotations,
        }
    }

    pub fn draw(&self, frame: &mut nanovg::Frame, _width: f32, _height: f32) {
        let font = nanovg::Font::from_file(
            &frame.context(),
            "Roboto-Regular",
            "resources/Roboto-Regular.ttf",
        )
        .unwrap(); // TODO
        frame.text(
            font,
            (0.0, 0.0),
            "FOOOOOO",
            nanovg::TextOptions {
                color: nanovg::Color::from_rgb(255, 255, 255),
                ..Default::default()
            },
        );
    }
}

// GameInfo
// <bold>Black</bold>\t captures
// PB (BR)
// <bold>White</bold>\t captures
// PW (WR)
// ----------------------------------
// <bold>N</bold>
// <bold>Komi</bold> KM
// <bold>Handicap</bold> HA
// <bold>Result</bold> RE
// <italic>TE, DO, BM, IT, DM, GB, GW, UC, HO</italic>
//
struct PlayerInfo {
    name: Option<String>,
    rank: Option<String>,
}

// impl std::fmt::Display for PlayerInfo {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

//     }
// }

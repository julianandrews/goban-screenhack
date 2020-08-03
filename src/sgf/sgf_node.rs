use super::props::SgfProp;

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

#[derive(Clone, Debug)]
pub struct SgfNode {
    pub properties: Vec<SgfProp>,
    pub children: Vec<SgfNode>,
}

impl SgfNode {
    pub fn get_size(&self) -> Option<(u8, u8)> {
        self.properties.iter().filter_map(|p| match p {
            SgfProp::SZ(size) => Some(size.clone()),
            _ => None
        }).next()
    }

    pub fn get_move(&self) -> Option<Stone> {
        for prop in self.properties.iter() {
            match prop {
                SgfProp::B(point) => return Some(Stone {
                    x: point.x,
                    y: point.y,
                    color: StoneColor::Black,
                }),
                SgfProp::W(point) => return Some(Stone {
                    x: point.x,
                    y: point.y,
                    color: StoneColor::White,
                }),
                _ => {},
            }
        }

        None
    }
}

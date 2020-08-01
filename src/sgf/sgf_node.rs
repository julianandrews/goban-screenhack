use super::props::{SgfProp, Point};

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Debug)]
pub struct SgfNode {
    pub properties: Vec<SgfProp>,
    pub children: Vec<SgfNode>,
}

impl SgfNode {
    fn get_size(&self) -> Option<(u8, u8)> {
        Some((13, 13))
    }

    fn get_move(&self) -> Option<(Point, Color)> {
        None
    //     // TODO: Handle B and W on same prop
    //     if let Ok(s) = self.0.get_point("B") {
    //         let point: Move = s.parse()?;
    //         Ok(Some(Stone { x: point.x, y: point.y, color: StoneColor::Black }))
    //     } else if let Ok(s) = self.0.get_point("W") {
    //         let point: Move = s.parse()?;
    //         Ok(Some(Stone { x: point.x, y: point.y, color: StoneColor::White }))
    //     } else {
    //         Ok(None)
    //     }
    }
}

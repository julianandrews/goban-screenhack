use super::props::SgfProp;

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
}

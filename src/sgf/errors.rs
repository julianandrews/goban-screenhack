#[derive(Debug)]
pub enum SgfParseError {
    InvalidGameTree,
    InvalidNode,
    InvalidProperty,
}

impl std::fmt::Display for SgfParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SgfParseError::InvalidGameTree => write!(f, "Invalid GameTree"),
            SgfParseError::InvalidNode => write!(f, "Invalid Node"),
            SgfParseError::InvalidProperty => write!(f, "Invalid Property"),
        }
    }
}

impl std::error::Error for SgfParseError {}

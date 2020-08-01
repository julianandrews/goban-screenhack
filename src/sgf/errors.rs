#[derive(Debug)]
pub enum SgfParseError {
    InvalidSgf,
    InvalidGameTree,
    InvalidNode,
    InvalidProperty,
    InvalidString,
}

impl std::fmt::Display for SgfParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SgfParseError::InvalidSgf => write!(f, "Invalid Sgf"),
            SgfParseError::InvalidGameTree => write!(f, "Invalid GameTree"),
            SgfParseError::InvalidNode => write!(f, "Invalid Node"),
            SgfParseError::InvalidProperty => write!(f, "Invalid Property"),
            SgfParseError::InvalidString => write!(f, "Invalid String"),
        }
    }
}

impl std::error::Error for SgfParseError {}

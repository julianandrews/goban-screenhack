use super::SgfParseError;

#[derive(Copy, Clone, Debug)]
pub enum Double {
    One,
    Two,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

// TODO: Handle all properties
#[derive(Clone, Debug)]
pub enum SgfProp {
    // Move Properties
    B(Point),
    KO,
    MN(i64),
    W(Point),
    // Setup Properties (illegal to place two colors on one point)
    // AB(PointList),
    // AE(PointList),
    // AW(PointList),
    // Node Annotation properties
    C(String),
    DM(Double),
    GB(Double),
    GW(Double),
    HO(Double),
    N(String),
    UC(Double),
    V(f64),
    // Move annotation properties (illegal without a move in the node)
    BM(Double),
    // DO,
    // IT,
    TE(Double),
    // Markup Properties (illegal to have more than one on a point)
    // AR(Vec<ComposedPoint>),
    // CR(PointList),
    // DD(EListPoint),
    // LB
    // LN
    // MA(PointList),
    // SL(PointList),
    // SQ(PointList),
    // TR(PointList),
    // Root Properties
    // AP
    CA(String),
    FF(i64), // range 1-4
    GM(i64), // range 1-16, only handle Go = 1!
    ST(i64), // range 0-3
    // SZ
    // Game info properties
    HA(i64), // >=2, AB should be set within same node
    KM(f64),
    AN(String),
    BR(String),
    BT(String),
    CP(String),
    DT(String),
    EV(String),
    GN(String),
    GC(String),
    ON(String),
    OT(String),
    PB(String),
    PC(String),
    PW(String),
    RE(String),
    RO(String),
    RU(String),
    SO(String),
    TM(f64),
    US(String),
    WR(String),
    WT(String),
    // Timing Properties
    BL(f64),
    OB(i64),
    OW(i64),
    WL(f64),
    // Miscellaneous properties
    // FG
    PM(i64), // range 1-2
    // VW(EListPoint),
    // TB
    // TW
    Unknown(String, Vec<String>),
}

impl SgfProp {
    pub fn new(ident: String, values: Vec<String>) -> Result<SgfProp, SgfParseError> {
        match &ident[..] {
            "B" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::B(value.parse()?))
            }
            "KO" => {
                get_no_value(&values)?;
                Ok(SgfProp::KO)
            }
            "MN" => {
                let value = get_single_value(&values)?;
                let value = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::MN(value))
            }
            "W" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::W(value.parse()?))
            }
            "C" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::C(value.to_string()))
            }
            "DM" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::DM(value.parse()?))
            }
            "GB" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::GB(value.parse()?))
            }
            "GW" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::GW(value.parse()?))
            }
            "HO" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::HO(value.parse()?))
            }
            "N" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::N(value.to_string()))
            }
            "UC" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::UC(value.parse()?))
            }
            "V" => {
                let value = get_single_value(&values)?;
                let value: f64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::V(value))
            }
            "BM" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::BM(value.parse()?))
            }
            "TE" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::TE(value.parse()?))
            }
            "CA" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::CA(value.to_string()))
            }
            "FF" => {
                let value = get_single_value(&values)?;
                let value: i64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                if value < 0 || value > 4 {
                    Err(SgfParseError::InvalidProperty)?;
                }
                Ok(SgfProp::FF(value))
            }
            "GM" => {
                let value = get_single_value(&values)?;
                let value: i64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                // Only Go is supported
                if value != 1 {
                    Err(SgfParseError::InvalidProperty)?;
                }
                Ok(SgfProp::GM(value))
            }
            "ST" => {
                let value = get_single_value(&values)?;
                let value: i64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                if value < 0 || value > 3 {
                    Err(SgfParseError::InvalidProperty)?;
                }
                Ok(SgfProp::ST(value))
            }
            "HA" => {
                let value = get_single_value(&values)?;
                let value: i64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                if !value >= 2 {
                    Err(SgfParseError::InvalidProperty)?;
                }
                Ok(SgfProp::HA(value))
            }
            "KM" => {
                let value = get_single_value(&values)?;
                let value: f64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::KM(value))
            }
            "AN" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::AN(value.to_string()))
            }
            "BR" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::BR(value.to_string()))
            }
            "BT" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::BT(value.to_string()))
            }
            "CP" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::CP(value.to_string()))
            }
            "DT" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::DT(value.to_string()))
            }
            "EV" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::EV(value.to_string()))
            }
            "GN" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::GN(value.to_string()))
            }
            "GC" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::GC(value.to_string()))
            }
            "ON" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::ON(value.to_string()))
            }
            "OT" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::OT(value.to_string()))
            }
            "PB" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::PB(value.to_string()))
            }
            "PC" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::PC(value.to_string()))
            }
            "PW" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::PW(value.to_string()))
            }
            "RE" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::RE(value.to_string()))
            }
            "RO" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::RO(value.to_string()))
            }
            "RU" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::RU(value.to_string()))
            }
            "SO" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::SO(value.to_string()))
            }
            "TM" => {
                let value = get_single_value(&values)?;
                let value: f64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::TM(value))
            }
            "US" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::US(value.to_string()))
            }
            "WR" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::WR(value.to_string()))
            }
            "WT" => {
                let value = get_single_value(&values)?;
                Ok(SgfProp::WT(value.to_string()))
            }
            "BL" => {
                let value = get_single_value(&values)?;
                let value: f64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::BL(value))
            }
            "OB" => {
                let value = get_single_value(&values)?;
                let value: i64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::OB(value))
            }
            "OW" => {
                let value = get_single_value(&values)?;
                let value: i64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::OW(value))
            }
            "WL" => {
                let value = get_single_value(&values)?;
                let value: f64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                Ok(SgfProp::WL(value))
            }
            "PM" => {
                let value = get_single_value(&values)?;
                let value: i64 = value.parse().map_err(|_| SgfParseError::InvalidProperty)?;
                if value < 1 || value > 2 {
                    Err(SgfParseError::InvalidProperty)?;
                }
                Ok(SgfProp::PM(value))
            }
            _ => Ok(SgfProp::Unknown(ident, values)),
        }
    }

    pub fn prop_ident(&self) -> &str {
        match self {
            SgfProp::B(_) => "B",
            SgfProp::W(_) => "W",
            SgfProp::Unknown(ident, _) => ident,
            _ => unimplemented!(), // TODO
        }
    }
}

fn get_no_value(values: &Vec<String>) -> Result<(), SgfParseError> {
    if !values.is_empty() {
        Err(SgfParseError::InvalidProperty)?;
    }
    Ok(())
}

fn get_single_value(values: &Vec<String>) -> Result<&str, SgfParseError> {
    if values.len() != 1 {
        Err(SgfParseError::InvalidProperty)?;
    }
    Ok(&values[0])
}

impl std::str::FromStr for Point {
    type Err = SgfParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(SgfParseError::InvalidProperty);
        }

        fn map_char(c: char) -> Result<u8, SgfParseError> {
            if c.is_ascii_lowercase() {
                Ok(c as u8 - 'a' as u8)
            } else if c.is_ascii_uppercase() {
                Ok(c as u8 - 'A' as u8)
            } else {
                Err(SgfParseError::InvalidProperty)
            }
        }

        Ok(Point {
            x: map_char(chars[0])?,
            y: map_char(chars[1])?,
        })
    }
}

impl std::str::FromStr for Double {
    type Err = SgfParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "1" {
            Ok(Double::One)
        } else if s == "2" {
            Ok(Double::Two)
        } else {
            Err(SgfParseError::InvalidProperty)
        }
    }
}

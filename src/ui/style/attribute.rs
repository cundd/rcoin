use std::fmt::{Display, Formatter, Result};
use super::StyleTrait;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Attribute {
    #[allow(unused)]
    /// Normal font style
    Normal = 0,

    #[allow(unused)]
    /// Normal Bold
    /// Bold/Bright
    BoldBright = 1,

    #[allow(unused)]
    /// Normal Dim
    /// Dim
    Dim = 2,

    #[allow(unused)]
    /// Normal Underlined
    /// Underlined
    Underlined = 4,

    #[allow(unused)]
    /// Normal Blink
    /// Blink 1)
    Blink = 5,

    #[allow(unused)]
    /// Normal inverted
    /// Reverse (invert the foreground and background colors)
    Reverse = 7,

    #[allow(unused)]
    /// Normal Hidden
    /// Hidden (useful for passwords)
    Hidden = 8,
}

impl StyleTrait for Attribute {
    fn from_u8(input: u8) -> Option<Self> {
        match input {
            0 => Some(Attribute::Normal),
            1 => Some(Attribute::BoldBright),
            2 => Some(Attribute::Dim),
            4 => Some(Attribute::Underlined),
            5 => Some(Attribute::Blink),
            7 => Some(Attribute::Reverse),
            8 => Some(Attribute::Hidden),
            _ => None,
        }
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\u{1b}[{}m", *self as u8)
    }
}

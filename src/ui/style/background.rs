use std::fmt::{Display, Formatter, Result};
use super::StyleTrait;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Background {
    #[allow(unused)]
    /// Default DefaultCode	Color	Example	Preview background
    /// Default background color
    Default = 49,

    #[allow(unused)]
    /// Default Black background
    /// Black
    Black = 40,

    #[allow(unused)]
    /// Default Red background
    /// Red
    Red = 41,

    #[allow(unused)]
    /// Default Green background
    /// Green
    Green = 42,

    #[allow(unused)]
    /// Default Yellow background
    /// Yellow
    Yellow = 43,

    #[allow(unused)]
    /// Default Blue background
    /// Blue
    Blue = 44,

    #[allow(unused)]
    /// Default Magenta background
    /// Magenta
    Magenta = 45,

    #[allow(unused)]
    /// Default Cyan background
    /// Cyan
    Cyan = 46,

    #[allow(unused)]
    /// Default Light gray background
    /// Light gray
    LightGray = 47,

    #[allow(unused)]
    /// Default Dark gray background
    /// Dark gray
    DarkGray = 100,

    #[allow(unused)]
    /// Default Light red background
    /// Light red
    LightRed = 101,

    #[allow(unused)]
    /// Default Light green background
    /// Light green
    LightGreen = 102,

    #[allow(unused)]
    /// Default Light yellow background
    /// Light yellow
    LightYellow = 103,

    #[allow(unused)]
    /// Default Light blue background
    /// Light blue
    LightBlue = 104,

    #[allow(unused)]
    /// Default Light magenta background
    /// Light magenta
    LightMagenta = 105,

    #[allow(unused)]
    /// Default Light cyan background
    /// Light cyan
    LightCyan = 106,

    #[allow(unused)]
    /// Default White background
    /// White
    White = 107,
}

impl StyleTrait for Background {
    fn from_u8(input: u8) -> Option<Self> {
        match input {
            49 => Some(Background::Default),
            40 => Some(Background::Black),
            41 => Some(Background::Red),
            42 => Some(Background::Green),
            43 => Some(Background::Yellow),
            44 => Some(Background::Blue),
            45 => Some(Background::Magenta),
            46 => Some(Background::Cyan),
            47 => Some(Background::LightGray),
            100 => Some(Background::DarkGray),
            101 => Some(Background::LightRed),
            102 => Some(Background::LightGreen),
            103 => Some(Background::LightYellow),
            104 => Some(Background::LightBlue),
            105 => Some(Background::LightMagenta),
            106 => Some(Background::LightCyan),
            107 => Some(Background::White),
            _ => None,
        }
    }
}

impl Display for Background {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\u{1b}[{}m", *self as u8)
    }
}

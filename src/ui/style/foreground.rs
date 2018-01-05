use std::fmt::{Display, Formatter, Result};
use super::StyleTrait;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Foreground {
    #[allow(unused)]
    /// Default DefaultCode	Color	Example	Preview
    /// Default foreground color
    Default = 39,

    #[allow(unused)]
    /// Default Black
    /// Black
    Black = 30,

    #[allow(unused)]
    /// Default Red
    /// Red
    Red = 31,

    #[allow(unused)]
    /// Default Green
    /// Green
    Green = 32,

    #[allow(unused)]
    /// Default Yellow
    /// Yellow
    Yellow = 33,

    #[allow(unused)]
    /// Default Blue
    /// Blue
    Blue = 34,

    #[allow(unused)]
    /// Default Magenta
    /// Magenta
    Magenta = 35,

    #[allow(unused)]
    /// Default Cyan
    /// Cyan
    Cyan = 36,

    #[allow(unused)]
    /// Default Light gray
    /// Light gray
    LightGray = 37,

    #[allow(unused)]
    /// Default Dark gray
    /// Dark gray
    DarkGray = 90,

    #[allow(unused)]
    /// Default Light red
    /// Light red
    LightRed = 91,

    #[allow(unused)]
    /// Default Light green
    /// Light green
    LightGreen = 92,

    #[allow(unused)]
    /// Default Light yellow
    /// Light yellow
    LightYellow = 93,

    #[allow(unused)]
    /// Default Light blue
    /// Light blue
    LightBlue = 94,

    #[allow(unused)]
    /// Default Light magenta
    /// Light magenta
    LightMagenta = 95,

    #[allow(unused)]
    /// Default Light cyan
    /// Light cyan
    LightCyan = 96,

    #[allow(unused)]
    /// Default White
    /// White
    White = 97,
}

impl StyleTrait for Foreground {
    fn from_u8(input: u8) -> Option<Self> {
        match input {
            39 => Some(Foreground::Default),
            30 => Some(Foreground::Black),
            31 => Some(Foreground::Red),
            32 => Some(Foreground::Green),
            33 => Some(Foreground::Yellow),
            34 => Some(Foreground::Blue),
            35 => Some(Foreground::Magenta),
            36 => Some(Foreground::Cyan),
            37 => Some(Foreground::LightGray),
            90 => Some(Foreground::DarkGray),
            91 => Some(Foreground::LightRed),
            92 => Some(Foreground::LightGreen),
            93 => Some(Foreground::LightYellow),
            94 => Some(Foreground::LightBlue),
            95 => Some(Foreground::LightMagenta),
            96 => Some(Foreground::LightCyan),
            97 => Some(Foreground::White),
            _ => None,
        }
    }
}

impl Display for Foreground {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\u{1b}[{}m", *self as u8)
    }
}

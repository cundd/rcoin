use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Style {
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

//    #[allow(unused)]
//    /// Normal Text
//    /// Reset all attributes
//    ResetAll = 0,
//
//    #[allow(unused)]
//    /// Normal Bold Normal
//    /// Reset bold/bright
//    ResetBold = 21,
//
//    #[allow(unused)]
//    /// Normal Dim Normal
//    /// Reset dim
//    ResetDim = 22,
//
//    #[allow(unused)]
//    /// Normal Underlined Normal
//    /// Reset underlined
//    ResetUnderlined = 24,
//
//    #[allow(unused)]
//    /// Normal Blink Normal
//    /// Reset blink
//    ResetBlink = 25,
//
//    #[allow(unused)]
//    /// Normal inverted Normal
//    /// Reset reverse
//    ResetReverse = 27,
//
//    #[allow(unused)]
//    /// Normal Hidden Normal
//    /// Reset hidden
//    ResetHidden = 28,

    #[allow(unused)]
    /// Default DefaultCode	Color	Example	Preview
    /// Default foreground color
    DefaultForeground = 39,

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

    #[allow(unused)]
    /// Default DefaultCode	Color	Example	Preview background
    /// Default background color
    BgDefaultBackground = 49,

    #[allow(unused)]
    /// Default Black background
    /// Black
    BgBlack = 40,

    #[allow(unused)]
    /// Default Red background
    /// Red
    BgRed = 41,

    #[allow(unused)]
    /// Default Green background
    /// Green
    BgGreen = 42,

    #[allow(unused)]
    /// Default Yellow background
    /// Yellow
    BgYellow = 43,

    #[allow(unused)]
    /// Default Blue background
    /// Blue
    BgBlue = 44,

    #[allow(unused)]
    /// Default Magenta background
    /// Magenta
    BgMagenta = 45,

    #[allow(unused)]
    /// Default Cyan background
    /// Cyan
    BgCyan = 46,

    #[allow(unused)]
    /// Default Light gray background
    /// Light gray
    BgLightGray = 47,

    #[allow(unused)]
    /// Default Dark gray background
    /// Dark gray
    BgDarkGray = 100,

    #[allow(unused)]
    /// Default Light red background
    /// Light red
    BgLightRed = 101,

    #[allow(unused)]
    /// Default Light green background
    /// Light green
    BgLightGreen = 102,

    #[allow(unused)]
    /// Default Light yellow background
    /// Light yellow
    BgLightYellow = 103,

    #[allow(unused)]
    /// Default Light blue background
    /// Light blue
    BgLightBlue = 104,

    #[allow(unused)]
    /// Default Light magenta background
    /// Light magenta
    BgLightMagenta = 105,

    #[allow(unused)]
    /// Default Light cyan background
    /// Light cyan
    BgLightCyan = 106,

    #[allow(unused)]
    /// Default White background
    /// White
    BgWhite = 107,
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", *self as u8)
    }
}

/// Wrap the message into the given style and reset with the `reset`
#[inline]
pub fn wrap<T: ::std::fmt::Display>(message: T, style: Style, reset: Style) -> String {
    format!(
        "{}[{}m{}{}[{}m",
        27 as char,
        style,
        message,
        27 as char,
        reset
    )
}

impl Style {
    pub fn from_str(input: &str) -> Option<Self> {
        match input.parse::<u8>() {
            Ok(int) => Self::from_u8(int),
            Err(_) => None,
        }
    }
    pub fn from_u8(input: u8) -> Option<Self> {
        match input {
            0 => Some(Style::Normal),
            1 => Some(Style::BoldBright),
            2 => Some(Style::Dim),
            4 => Some(Style::Underlined),
            5 => Some(Style::Blink),
            7 => Some(Style::Reverse),
            8 => Some(Style::Hidden),
            // 0 => Some(Style::ResetAll),
            // 21 => Some(Style::ResetBold),
            // 22 => Some(Style::ResetDim),
            // 24 => Some(Style::ResetUnderlined),
            // 25 => Some(Style::ResetBlink),
            // 27 => Some(Style::ResetReverse),
            // 28 => Some(Style::ResetHidden),
            39 => Some(Style::DefaultForeground),
            30 => Some(Style::Black),
            31 => Some(Style::Red),
            32 => Some(Style::Green),
            33 => Some(Style::Yellow),
            34 => Some(Style::Blue),
            35 => Some(Style::Magenta),
            36 => Some(Style::Cyan),
            37 => Some(Style::LightGray),
            90 => Some(Style::DarkGray),
            91 => Some(Style::LightRed),
            92 => Some(Style::LightGreen),
            93 => Some(Style::LightYellow),
            94 => Some(Style::LightBlue),
            95 => Some(Style::LightMagenta),
            96 => Some(Style::LightCyan),
            97 => Some(Style::White),
            49 => Some(Style::BgDefaultBackground),
            40 => Some(Style::BgBlack),
            41 => Some(Style::BgRed),
            42 => Some(Style::BgGreen),
            43 => Some(Style::BgYellow),
            44 => Some(Style::BgBlue),
            45 => Some(Style::BgMagenta),
            46 => Some(Style::BgCyan),
            47 => Some(Style::BgLightGray),
            100 => Some(Style::BgDarkGray),
            101 => Some(Style::BgLightRed),
            102 => Some(Style::BgLightGreen),
            103 => Some(Style::BgLightYellow),
            104 => Some(Style::BgLightBlue),
            105 => Some(Style::BgLightMagenta),
            106 => Some(Style::BgLightCyan),
            107 => Some(Style::BgWhite),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_ut8_test() {
        assert_eq!(Some(Style::Normal), Style::from_u8(0));
        assert_eq!(Some(Style::BoldBright), Style::from_u8(1));
        assert_eq!(Some(Style::Dim), Style::from_u8(2));
        assert_eq!(Some(Style::Underlined), Style::from_u8(4));
        assert_eq!(Some(Style::Blink), Style::from_u8(5));
        assert_eq!(Some(Style::Reverse), Style::from_u8(7));
        assert_eq!(Some(Style::Hidden), Style::from_u8(8));
        assert_eq!(Some(Style::DefaultForeground), Style::from_u8(39));
        assert_eq!(Some(Style::Black), Style::from_u8(30));
        assert_eq!(Some(Style::Red), Style::from_u8(31));
        assert_eq!(Some(Style::Green), Style::from_u8(32));
        assert_eq!(Some(Style::Yellow), Style::from_u8(33));
        assert_eq!(Some(Style::Blue), Style::from_u8(34));
        assert_eq!(Some(Style::Magenta), Style::from_u8(35));
        assert_eq!(Some(Style::Cyan), Style::from_u8(36));
        assert_eq!(Some(Style::LightGray), Style::from_u8(37));
        assert_eq!(Some(Style::DarkGray), Style::from_u8(90));
        assert_eq!(Some(Style::LightRed), Style::from_u8(91));
        assert_eq!(Some(Style::LightGreen), Style::from_u8(92));
        assert_eq!(Some(Style::LightYellow), Style::from_u8(93));
        assert_eq!(Some(Style::LightBlue), Style::from_u8(94));
        assert_eq!(Some(Style::LightMagenta), Style::from_u8(95));
        assert_eq!(Some(Style::LightCyan), Style::from_u8(96));
        assert_eq!(Some(Style::White), Style::from_u8(97));
        assert_eq!(Some(Style::BgDefaultBackground), Style::from_u8(49));
        assert_eq!(Some(Style::BgBlack), Style::from_u8(40));
        assert_eq!(Some(Style::BgRed), Style::from_u8(41));
        assert_eq!(Some(Style::BgGreen), Style::from_u8(42));
        assert_eq!(Some(Style::BgYellow), Style::from_u8(43));
        assert_eq!(Some(Style::BgBlue), Style::from_u8(44));
        assert_eq!(Some(Style::BgMagenta), Style::from_u8(45));
        assert_eq!(Some(Style::BgCyan), Style::from_u8(46));
        assert_eq!(Some(Style::BgLightGray), Style::from_u8(47));
        assert_eq!(Some(Style::BgDarkGray), Style::from_u8(100));
        assert_eq!(Some(Style::BgLightRed), Style::from_u8(101));
        assert_eq!(Some(Style::BgLightGreen), Style::from_u8(102));
        assert_eq!(Some(Style::BgLightYellow), Style::from_u8(103));
        assert_eq!(Some(Style::BgLightBlue), Style::from_u8(104));
        assert_eq!(Some(Style::BgLightMagenta), Style::from_u8(105));
        assert_eq!(Some(Style::BgLightCyan), Style::from_u8(106));
        assert_eq!(Some(Style::BgWhite), Style::from_u8(107));
        assert_eq!(None, Style::from_u8(140));
    }

    #[test]
    fn from_str_test() {
        assert_eq!(Some(Style::Normal), Style::from_str("0"));
        assert_eq!(Some(Style::BoldBright), Style::from_str("1"));
        assert_eq!(Some(Style::Dim), Style::from_str("2"));
        assert_eq!(Some(Style::Underlined), Style::from_str("4"));
        assert_eq!(Some(Style::Blink), Style::from_str("5"));
        assert_eq!(Some(Style::Reverse), Style::from_str("7"));
        assert_eq!(Some(Style::Hidden), Style::from_str("8"));
        assert_eq!(Some(Style::DefaultForeground), Style::from_str("39"));
        assert_eq!(Some(Style::Black), Style::from_str("30"));
        assert_eq!(Some(Style::Red), Style::from_str("31"));
        assert_eq!(Some(Style::Green), Style::from_str("32"));
        assert_eq!(Some(Style::Yellow), Style::from_str("33"));
        assert_eq!(Some(Style::Blue), Style::from_str("34"));
        assert_eq!(Some(Style::Magenta), Style::from_str("35"));
        assert_eq!(Some(Style::Cyan), Style::from_str("36"));
        assert_eq!(Some(Style::LightGray), Style::from_str("37"));
        assert_eq!(Some(Style::DarkGray), Style::from_str("90"));
        assert_eq!(Some(Style::LightRed), Style::from_str("91"));
        assert_eq!(Some(Style::LightGreen), Style::from_str("92"));
        assert_eq!(Some(Style::LightYellow), Style::from_str("93"));
        assert_eq!(Some(Style::LightBlue), Style::from_str("94"));
        assert_eq!(Some(Style::LightMagenta), Style::from_str("95"));
        assert_eq!(Some(Style::LightCyan), Style::from_str("96"));
        assert_eq!(Some(Style::White), Style::from_str("97"));
        assert_eq!(Some(Style::BgDefaultBackground), Style::from_str("49"));
        assert_eq!(Some(Style::BgBlack), Style::from_str("40"));
        assert_eq!(Some(Style::BgRed), Style::from_str("41"));
        assert_eq!(Some(Style::BgGreen), Style::from_str("42"));
        assert_eq!(Some(Style::BgYellow), Style::from_str("43"));
        assert_eq!(Some(Style::BgBlue), Style::from_str("44"));
        assert_eq!(Some(Style::BgMagenta), Style::from_str("45"));
        assert_eq!(Some(Style::BgCyan), Style::from_str("46"));
        assert_eq!(Some(Style::BgLightGray), Style::from_str("47"));
        assert_eq!(Some(Style::BgDarkGray), Style::from_str("100"));
        assert_eq!(Some(Style::BgLightRed), Style::from_str("101"));
        assert_eq!(Some(Style::BgLightGreen), Style::from_str("102"));
        assert_eq!(Some(Style::BgLightYellow), Style::from_str("103"));
        assert_eq!(Some(Style::BgLightBlue), Style::from_str("104"));
        assert_eq!(Some(Style::BgLightMagenta), Style::from_str("105"));
        assert_eq!(Some(Style::BgLightCyan), Style::from_str("106"));
        assert_eq!(Some(Style::BgWhite), Style::from_str("107"));
        assert_eq!(None, Style::from_str("140"));
    }
}

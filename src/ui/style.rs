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

#[allow(unused)]
pub fn style(message: &str, color: u8) -> String {
    format!(
        "{}[{}m{}{}[{}m",
        27 as char,
        color,
        message,
        27 as char,
        RESET_ALL
    )
}

#[allow(unused)]
/// Output the text in red
pub fn reverse(message: &str) -> String {
    style(message, REVERSE)
}

#[allow(unused)]
/// Output the text in red
pub fn red(message: &str) -> String {
    style(message, RED)
}

#[allow(unused)]
/// Output the text in green
pub fn green(message: &str) -> String {
    style(message, GREEN)
}

#[allow(unused)]
/// Output the text in yellow
pub fn yellow(message: &str) -> String {
    style(message, YELLOW)
}

#[allow(unused)]
/// Output the text in blue
pub fn blue(message: &str) -> String {
    style(message, BLUE)
}

#[allow(unused)]
/// Output the text in magenta
pub fn magenta(message: &str) -> String {
    style(message, MAGENTA)
}

#[allow(unused)]
/// Output the text in cyan
pub fn cyan(message: &str) -> String {
    style(message, CYAN)
}

#[allow(unused)]
/// Output the text in light_gray
pub fn light_gray(message: &str) -> String {
    style(message, LIGHT_GRAY)
}

#[allow(unused)]
/// Output the text in dark_gray
pub fn dark_gray(message: &str) -> String {
    style(message, DARK_GRAY)
}

#[allow(unused)]
/// Output the text in light_red
pub fn light_red(message: &str) -> String {
    style(message, LIGHT_RED)
}

#[allow(unused)]
/// Output the text in light_green
pub fn light_green(message: &str) -> String {
    style(message, LIGHT_GREEN)
}

#[allow(unused)]
/// Output the text in light_yellow
pub fn light_yellow(message: &str) -> String {
    style(message, LIGHT_YELLOW)
}

#[allow(unused)]
/// Output the text in light_blue
pub fn light_blue(message: &str) -> String {
    style(message, LIGHT_BLUE)
}

#[allow(unused)]
/// Output the text in light_magenta
pub fn light_magenta(message: &str) -> String {
    style(message, LIGHT_MAGENTA)
}

#[allow(unused)]
/// Output the text in light_cyan
pub fn light_cyan(message: &str) -> String {
    style(message, LIGHT_CYAN)
}

#[allow(unused)]
/// Output the text in white
pub fn white(message: &str) -> String {
    style(message, WHITE)
}


#[allow(unused)]
/// Draw the background default_background
pub fn bg_default_background(message: &str) -> String {
    style(message, BG_DEFAULT_BACKGROUND)
}

#[allow(unused)]
/// Draw the background black
pub fn bg_black(message: &str) -> String {
    style(message, BG_BLACK)
}

#[allow(unused)]
/// Draw the background red
pub fn bg_red(message: &str) -> String {
    style(message, BG_RED)
}

#[allow(unused)]
/// Draw the background green
pub fn bg_green(message: &str) -> String {
    style(message, BG_GREEN)
}

#[allow(unused)]
/// Draw the background yellow
pub fn bg_yellow(message: &str) -> String {
    style(message, BG_YELLOW)
}

#[allow(unused)]
/// Draw the background blue
pub fn bg_blue(message: &str) -> String {
    style(message, BG_BLUE)
}

#[allow(unused)]
/// Draw the background magenta
pub fn bg_magenta(message: &str) -> String {
    style(message, BG_MAGENTA)
}

#[allow(unused)]
/// Draw the background cyan
pub fn bg_cyan(message: &str) -> String {
    style(message, BG_CYAN)
}

#[allow(unused)]
/// Draw the background light_gray
pub fn bg_light_gray(message: &str) -> String {
    style(message, BG_LIGHT_GRAY)
}

#[allow(unused)]
/// Draw the background dark_gray
pub fn bg_dark_gray(message: &str) -> String {
    style(message, BG_DARK_GRAY)
}

#[allow(unused)]
/// Draw the background light_red
pub fn bg_light_red(message: &str) -> String {
    style(message, BG_LIGHT_RED)
}

#[allow(unused)]
/// Draw the background light_green
pub fn bg_light_green(message: &str) -> String {
    style(message, BG_LIGHT_GREEN)
}

#[allow(unused)]
/// Draw the background light_yellow
pub fn bg_light_yellow(message: &str) -> String {
    style(message, BG_LIGHT_YELLOW)
}

#[allow(unused)]
/// Draw the background light_blue
pub fn bg_light_blue(message: &str) -> String {
    style(message, BG_LIGHT_BLUE)
}

#[allow(unused)]
/// Draw the background light_magenta
pub fn bg_light_magenta(message: &str) -> String {
    style(message, BG_LIGHT_MAGENTA)
}

#[allow(unused)]
/// Draw the background light_cyan
pub fn bg_light_cyan(message: &str) -> String {
    style(message, BG_LIGHT_CYAN)
}

#[allow(unused)]
/// Draw the background white
pub fn bg_white(message: &str) -> String {
    style(message, BG_WHITE)
}


#[allow(unused)]
/// Normal Bold
/// Bold/Bright
pub const BOLD_BRIGHT: u8 = 1;

#[allow(unused)]
/// Normal Dim
/// Dim
pub const DIM: u8 = 2;

#[allow(unused)]
/// Normal Underlined
/// Underlined
pub const UNDERLINED: u8 = 4;

#[allow(unused)]
/// Normal Blink
/// Blink 1)
pub const BLINK: u8 = 5;

#[allow(unused)]
/// Normal inverted
/// Reverse (invert the foreground and background colors)
pub const REVERSE: u8 = 7;

#[allow(unused)]
/// Normal Hidden
/// Hidden (useful for passwords)
pub const HIDDEN: u8 = 8;


#[allow(unused)]
/// Normal Text
/// Reset all attributes
pub const RESET_ALL: u8 = 0;

#[allow(unused)]
/// Normal Bold Normal
/// Reset bold/bright
pub const RESET_BOLD: u8 = 21;

#[allow(unused)]
/// Normal Dim Normal
/// Reset dim
pub const RESET_DIM: u8 = 22;

#[allow(unused)]
/// Normal Underlined Normal
/// Reset underlined
pub const RESET_UNDERLINED: u8 = 24;

#[allow(unused)]
/// Normal Blink Normal
/// Reset blink
pub const RESET_BLINK: u8 = 25;

#[allow(unused)]
/// Normal inverted Normal
/// Reset reverse
pub const RESET_REVERSE: u8 = 27;

#[allow(unused)]
/// Normal Hidden Normal
/// Reset hidden
pub const RESET_HIDDEN: u8 = 28;


#[allow(unused)]
/// Default DefaultCode	Color	Example	Preview
/// Default foreground color
pub const DEFAULT_FOREGROUND: u8 = 39;

#[allow(unused)]
/// Default Black
/// Black
pub const BLACK: u8 = 30;

#[allow(unused)]
/// Default Red
/// Red
pub const RED: u8 = 31;

#[allow(unused)]
/// Default Green
/// Green
pub const GREEN: u8 = 32;

#[allow(unused)]
/// Default Yellow
/// Yellow
pub const YELLOW: u8 = 33;

#[allow(unused)]
/// Default Blue
/// Blue
pub const BLUE: u8 = 34;

#[allow(unused)]
/// Default Magenta
/// Magenta
pub const MAGENTA: u8 = 35;

#[allow(unused)]
/// Default Cyan
/// Cyan
pub const CYAN: u8 = 36;

#[allow(unused)]
/// Default Light gray
/// Light gray
pub const LIGHT_GRAY: u8 = 37;

#[allow(unused)]
/// Default Dark gray
/// Dark gray
pub const DARK_GRAY: u8 = 90;

#[allow(unused)]
/// Default Light red
/// Light red
pub const LIGHT_RED: u8 = 91;

#[allow(unused)]
/// Default Light green
/// Light green
pub const LIGHT_GREEN: u8 = 92;

#[allow(unused)]
/// Default Light yellow
/// Light yellow
pub const LIGHT_YELLOW: u8 = 93;

#[allow(unused)]
/// Default Light blue
/// Light blue
pub const LIGHT_BLUE: u8 = 94;

#[allow(unused)]
/// Default Light magenta
/// Light magenta
pub const LIGHT_MAGENTA: u8 = 95;

#[allow(unused)]
/// Default Light cyan
/// Light cyan
pub const LIGHT_CYAN: u8 = 96;

#[allow(unused)]
/// Default White
/// White
pub const WHITE: u8 = 97;


#[allow(unused)]
/// Default DefaultCode	Color	Example	Preview background
/// Default background color
pub const BG_DEFAULT_BACKGROUND: u8 = 49;

#[allow(unused)]
/// Default Black background
/// Black
pub const BG_BLACK: u8 = 40;

#[allow(unused)]
/// Default Red background
/// Red
pub const BG_RED: u8 = 41;

#[allow(unused)]
/// Default Green background
/// Green
pub const BG_GREEN: u8 = 42;

#[allow(unused)]
/// Default Yellow background
/// Yellow
pub const BG_YELLOW: u8 = 43;

#[allow(unused)]
/// Default Blue background
/// Blue
pub const BG_BLUE: u8 = 44;

#[allow(unused)]
/// Default Magenta background
/// Magenta
pub const BG_MAGENTA: u8 = 45;

#[allow(unused)]
/// Default Cyan background
/// Cyan
pub const BG_CYAN: u8 = 46;

#[allow(unused)]
/// Default Light gray background
/// Light gray
pub const BG_LIGHT_GRAY: u8 = 47;

#[allow(unused)]
/// Default Dark gray background
/// Dark gray
pub const BG_DARK_GRAY: u8 = 100;

#[allow(unused)]
/// Default Light red background
/// Light red
pub const BG_LIGHT_RED: u8 = 101;

#[allow(unused)]
/// Default Light green background
/// Light green
pub const BG_LIGHT_GREEN: u8 = 102;

#[allow(unused)]
/// Default Light yellow background
/// Light yellow
pub const BG_LIGHT_YELLOW: u8 = 103;

#[allow(unused)]
/// Default Light blue background
/// Light blue
pub const BG_LIGHT_BLUE: u8 = 104;

#[allow(unused)]
/// Default Light magenta background
/// Light magenta
pub const BG_LIGHT_MAGENTA: u8 = 105;

#[allow(unused)]
/// Default Light cyan background
/// Light cyan
pub const BG_LIGHT_CYAN: u8 = 106;

#[allow(unused)]
/// Default White background
/// White
pub const BG_WHITE: u8 = 107;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_test() {
        assert_eq!("\u{1b}[32mmy message\u{1b}[0m", style("my message", GREEN));
    }

    #[test]
    fn red_test() {
        assert_eq!("\u{1b}[31mmy message\u{1b}[0m", red("my message"));
    }

    #[test]
    fn green_test() {
        assert_eq!("\u{1b}[32mmy message\u{1b}[0m", green("my message"));
    }

    #[test]
    fn yellow_test() {
        assert_eq!("\u{1b}[33mmy message\u{1b}[0m", yellow("my message"));
    }

    #[test]
    fn blue_test() {
        assert_eq!("\u{1b}[34mmy message\u{1b}[0m", blue("my message"));
    }

    #[test]
    fn magenta_test() {
        assert_eq!("\u{1b}[35mmy message\u{1b}[0m", magenta("my message"));
    }

    #[test]
    fn cyan_test() {
        assert_eq!("\u{1b}[36mmy message\u{1b}[0m", cyan("my message"));
    }

    #[test]
    fn light_gray_test() {
        assert_eq!("\u{1b}[37mmy message\u{1b}[0m", light_gray("my message"));
    }

    #[test]
    fn dark_gray_test() {
        assert_eq!("\u{1b}[90mmy message\u{1b}[0m", dark_gray("my message"));
    }

    #[test]
    fn light_red_test() {
        assert_eq!("\u{1b}[91mmy message\u{1b}[0m", light_red("my message"));
    }

    #[test]
    fn light_green_test() {
        assert_eq!("\u{1b}[92mmy message\u{1b}[0m", light_green("my message"));
    }

    #[test]
    fn light_yellow_test() {
        assert_eq!("\u{1b}[93mmy message\u{1b}[0m", light_yellow("my message"));
    }

    #[test]
    fn light_blue_test() {
        assert_eq!("\u{1b}[94mmy message\u{1b}[0m", light_blue("my message"));
    }

    #[test]
    fn light_magenta_test() {
        assert_eq!("\u{1b}[95mmy message\u{1b}[0m", light_magenta("my message"));
    }

    #[test]
    fn light_cyan_test() {
        assert_eq!("\u{1b}[96mmy message\u{1b}[0m", light_cyan("my message"));
    }

    #[test]
    fn white_test() {
        assert_eq!("\u{1b}[97mmy message\u{1b}[0m", white("my message"));
    }

    #[test]
    fn draw_background_default_background_test() {
        assert_eq!("\u{1b}[49mmy message\u{1b}[0m", bg_default_background("my message"));
        assert_eq!("\u{1b}[49mmy message\u{1b}[0m", style("my message", BG_DEFAULT_BACKGROUND));
    }

    #[test]
    fn draw_background_black_test() {
        assert_eq!("\u{1b}[40mmy message\u{1b}[0m", bg_black("my message"));
        assert_eq!("\u{1b}[40mmy message\u{1b}[0m", style("my message", BG_BLACK));
    }

    #[test]
    fn draw_background_red_test() {
        assert_eq!("\u{1b}[41mmy message\u{1b}[0m", bg_red("my message"));
        assert_eq!("\u{1b}[41mmy message\u{1b}[0m", style("my message", BG_RED));
    }

    #[test]
    fn draw_background_green_test() {
        assert_eq!("\u{1b}[42mmy message\u{1b}[0m", bg_green("my message"));
        assert_eq!("\u{1b}[42mmy message\u{1b}[0m", style("my message", BG_GREEN));
    }

    #[test]
    fn draw_background_yellow_test() {
        assert_eq!("\u{1b}[43mmy message\u{1b}[0m", bg_yellow("my message"));
        assert_eq!("\u{1b}[43mmy message\u{1b}[0m", style("my message", BG_YELLOW));
    }

    #[test]
    fn draw_background_blue_test() {
        assert_eq!("\u{1b}[44mmy message\u{1b}[0m", bg_blue("my message"));
        assert_eq!("\u{1b}[44mmy message\u{1b}[0m", style("my message", BG_BLUE));
    }

    #[test]
    fn draw_background_magenta_test() {
        assert_eq!("\u{1b}[45mmy message\u{1b}[0m", bg_magenta("my message"));
        assert_eq!("\u{1b}[45mmy message\u{1b}[0m", style("my message", BG_MAGENTA));
    }

    #[test]
    fn draw_background_cyan_test() {
        assert_eq!("\u{1b}[46mmy message\u{1b}[0m", bg_cyan("my message"));
        assert_eq!("\u{1b}[46mmy message\u{1b}[0m", style("my message", BG_CYAN));
    }

    #[test]
    fn draw_background_light_gray_test() {
        assert_eq!("\u{1b}[47mmy message\u{1b}[0m", bg_light_gray("my message"));
        assert_eq!("\u{1b}[47mmy message\u{1b}[0m", style("my message", BG_LIGHT_GRAY));
    }

    #[test]
    fn draw_background_dark_gray_test() {
        assert_eq!("\u{1b}[100mmy message\u{1b}[0m", bg_dark_gray("my message"));
        assert_eq!("\u{1b}[100mmy message\u{1b}[0m", style("my message", BG_DARK_GRAY));
    }

    #[test]
    fn draw_background_light_red_test() {
        assert_eq!("\u{1b}[101mmy message\u{1b}[0m", bg_light_red("my message"));
        assert_eq!("\u{1b}[101mmy message\u{1b}[0m", style("my message", BG_LIGHT_RED));
    }

    #[test]
    fn draw_background_light_green_test() {
        assert_eq!("\u{1b}[102mmy message\u{1b}[0m", bg_light_green("my message"));
        assert_eq!("\u{1b}[102mmy message\u{1b}[0m", style("my message", BG_LIGHT_GREEN));
    }

    #[test]
    fn draw_background_light_yellow_test() {
        assert_eq!("\u{1b}[103mmy message\u{1b}[0m", bg_light_yellow("my message"));
        assert_eq!("\u{1b}[103mmy message\u{1b}[0m", style("my message", BG_LIGHT_YELLOW));
    }

    #[test]
    fn draw_background_light_blue_test() {
        assert_eq!("\u{1b}[104mmy message\u{1b}[0m", bg_light_blue("my message"));
        assert_eq!("\u{1b}[104mmy message\u{1b}[0m", style("my message", BG_LIGHT_BLUE));
    }

    #[test]
    fn draw_background_light_magenta_test() {
        assert_eq!("\u{1b}[105mmy message\u{1b}[0m", bg_light_magenta("my message"));
        assert_eq!("\u{1b}[105mmy message\u{1b}[0m", style("my message", BG_LIGHT_MAGENTA));
    }

    #[test]
    fn draw_background_light_cyan_test() {
        assert_eq!("\u{1b}[106mmy message\u{1b}[0m", bg_light_cyan("my message"));
        assert_eq!("\u{1b}[106mmy message\u{1b}[0m", style("my message", BG_LIGHT_CYAN));
    }

    #[test]
    fn draw_background_white_test() {
        assert_eq!("\u{1b}[107mmy message\u{1b}[0m", bg_white("my message"));
        assert_eq!("\u{1b}[107mmy message\u{1b}[0m", style("my message", BG_WHITE));
    }
}
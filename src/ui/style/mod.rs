mod attribute;
mod foreground;
mod background;

use std::fmt::Display;

pub use self::foreground::Foreground;
pub use self::attribute::Attribute;
pub use self::background::Background;

pub trait StyleTrait: Sized {
    fn from_str(input: &str) -> Option<Self> {
        match input.parse::<u8>() {
            Ok(int) => Self::from_u8(int),
            Err(_) => None,
        }
    }

    fn from_u8(input: u8) -> Option<Self>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Styles {
    pub foreground: Foreground,
    pub background: Background,
    pub attribute: Attribute,
}

impl Styles {
    pub fn normal() -> Self {
        Styles {
            foreground: Foreground::Default,
            background: Background::Default,
            attribute: Attribute::Normal,
        }
    }

    pub fn with_foreground(&self, foreground: Foreground) -> Self {
        let mut clone = self.clone();
        clone.foreground = foreground;

        clone
    }
    pub fn with_background(&self, background: Background) -> Self {
        let mut clone = self.clone();
        clone.background = background;

        clone
    }
    pub fn with_attribute(&self, attribute: Attribute) -> Self {
        let mut clone = self.clone();
        clone.attribute = attribute;

        clone
    }
}

impl StyleTrait for Styles {
    fn from_u8(input: u8) -> Option<Self> {
        if input <= 10 {
            if let Some(style) = Attribute::from_u8(input) {
                return Some(Styles::normal().with_attribute(style));
            }
        } else if input <= 97 {
            if let Some(style) = Foreground::from_u8(input) {
                return Some(Styles::normal().with_foreground(style));
            }
        } else if let Some(style) = Background::from_u8(input) {
            return Some(Styles::normal().with_background(style));
        }

        return None;
    }
}

/// Wrap the message into the given styles and reset with the `reset`
#[inline]
pub fn wrap<T: Display>(message: T, styles: Styles) -> String {
    format!("{}{}{}{}\u{1b}[0m",
            styles.attribute,
            styles.background,
            styles.foreground,
            message
    )
}

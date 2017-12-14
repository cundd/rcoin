#[derive(Debug, PartialEq)]
pub enum Mode {
    Truncate,
    Scale,
    ScaleX,
    ScaleY,
    ScaleDown,
    ScaleDownX,
    ScaleDownY,
}

impl Mode {
    pub fn from_str(mode: &str) -> Result<Self, ()> {
        match mode.as_ref() {
            "truncate" => Ok(Mode::Truncate),
            "scale" => Ok(Mode::Scale),
            "scale-x" => Ok(Mode::ScaleX),
            "scale-y" => Ok(Mode::ScaleY),
            "scale-down" => Ok(Mode::ScaleDown),
            "scale-down-x" => Ok(Mode::ScaleDownX),
            "scale-down-y" => Ok(Mode::ScaleDownY),
            _ => Err(()),
        }
    }
    pub fn from_option_str(mode: Option<String>) -> Result<Self, ()> {
        match mode {
            Some(mode) => Mode::from_str(&mode),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert!(Mode::from_str("invalid").is_err());
        assert!(Mode::from_str("").is_err());

        assert_eq!(Mode::Scale, Mode::from_str("scale").unwrap());
        assert_eq!(Mode::ScaleX, Mode::from_str("scale-x").unwrap());
        assert_eq!(Mode::ScaleY, Mode::from_str("scale-y").unwrap());
        assert_eq!(Mode::ScaleDown, Mode::from_str("scale-down").unwrap());
        assert_eq!(Mode::ScaleDownX, Mode::from_str("scale-down-x").unwrap());
        assert_eq!(Mode::ScaleDownY, Mode::from_str("scale-down-y").unwrap());
        assert_eq!(Mode::Truncate, Mode::from_str("truncate").unwrap());
    }
}
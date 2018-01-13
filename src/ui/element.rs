use super::style::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Element {
    pub character: char,
    pub styles: Styles,
}

#[allow(unused)]
impl Element {
    pub fn new(character: char, styles: Styles) -> Self {
        Element {
            character,
            styles,
        }
    }

    pub fn normal(character: char) -> Self {
        Self::new(character, Styles::normal())
    }

    pub fn blank() -> Self {
        Element { character: ' ', styles: Styles::normal() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_test() {
        assert_eq!(
            Element {
                character: 'c',
                styles: Styles::normal(),
            },
            Element::normal('c')
        );
    }

    #[test]
    fn blank_test() {
        assert_eq!(
            Element {
                character: ' ',
                styles: Styles::normal(),
            },
            Element::blank()
        );
    }
}
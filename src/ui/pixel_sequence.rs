use super::pixel::Pixel;
use super::style::*;

pub type PixelSequence = Vec<Pixel>;

pub trait PixelSequenceTrait {
    fn from_str(text: &str) -> Self;
}

impl PixelSequenceTrait for PixelSequence {
    fn from_str(text: &str) -> Self {
        let mut sequence = PixelSequence::with_capacity(text.len());
        let mut current_x = 0;
        let mut current_y = 0;
        let mut index = 0;
        let mut style = Styles::normal();
        let chars: Vec<char> = text.chars().collect();

        while let Some(character) = chars.get(index) {
            match *character {
                '\n' => {
                    sequence.push(Pixel::new('\n', current_x, current_y, style));
                    current_y += 1;
                    current_x = 0;
                    index += 1;

                    continue;
                }
                character @ _ if character.is_control() => {
                    let (offset, new_style) = consume_control_sequence(text, index);
                    style = new_style;
                    index += offset;
                }
                character @ _ => {
                    sequence.push(Pixel::new(character, current_x, current_y, style));
                    current_x += 1;
                }
            }

            index += 1;
        }

        sequence.shrink_to_fit();
        sequence
    }
}

#[inline]
fn consume_control_sequence<'a>(text: &str, start_index: usize) -> (usize, Styles) {
    let mut offset = 0;
    let mut style_string = String::new();

    while let Some(character) = text.chars().nth(start_index + offset) {
        match character {
            'm' => break,
            '0' ... '9' => style_string.push(character),
            _ => {}
        };
        offset += 1;
    }

    let style = Styles::from_str(&style_string).unwrap_or(Styles::normal());

    (offset, style)
}

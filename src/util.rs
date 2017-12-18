use std::borrow::Cow;

pub fn str_pad<'a>(input: &'a str, length: usize, fill: char) -> Cow<'a, str> {
    let input_length = input.chars().count();
    if input_length >= length {
        return Cow::Borrowed(input);
    }

    let char_count_diff = length - input_length;
    let capacity = input.len() + (char_count_diff * fill.len_utf8());

    let mut buf = String::with_capacity(capacity);

    buf.push_str(input);

    let mut i = char_count_diff;
    while i > 0 {
        buf.push(fill);
        i -= 1;
    }

    return Cow::Owned(buf);
}

pub fn str_left_pad<'a>(input: &'a str, length: usize, fill: char) -> Cow<'a, str> {
    let input_length = input.chars().count();
    if input_length >= length {
        return Cow::Borrowed(input);
    }

    let char_count_diff = length - input_length;
    let capacity = input.len() + (char_count_diff * fill.len_utf8());

    let mut buf = String::with_capacity(capacity);

    buf.push_str(input);

    let mut i = char_count_diff;
    while i > 0 {
        buf.insert_str(0, &fill.to_string());
        i -= 1;
    }

    return Cow::Owned(buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_pad_test() {
        assert_eq!("Hello     ", str_pad("Hello", 10, ' '));
        assert_eq!("Hello.....", str_pad("Hello", 10, '.'));
        assert_eq!(" Hello....", str_pad(" Hello", 10, '.'));
        assert_eq!("Hello💋💋💋💋💋", str_pad("Hello", 10, '💋'));
        assert_eq!("Hello\n\n\n\n\n", str_pad("Hello", 10, '\n'));
        assert_eq!("ö.........", str_pad("ö", 10, '.'));
        assert_eq!("💋.........", str_pad("💋", 10, '.'));

        assert_eq!("x ", str_pad("x ", 2, '.'));
        assert_eq!("x ------", str_pad("x ------", 2, '.'));
        assert_eq!("ö ", str_pad("ö ", 2, '.'));
        assert_eq!("ö ------", str_pad("ö ------", 2, '.'));
        assert_eq!("💋 ", str_pad("💋 ", 2, '.'));
        assert_eq!("💋 ------", str_pad("💋 ------", 2, '.'));
    }

    #[test]
    fn str_left_pad_test() {
        assert_eq!("     Hello", str_left_pad("Hello", 10, ' '));
        assert_eq!(".....Hello", str_left_pad("Hello", 10, '.'));
        assert_eq!(".... Hello", str_left_pad(" Hello", 10, '.'));
        assert_eq!("💋💋💋💋💋Hello", str_left_pad("Hello", 10, '💋'));
        assert_eq!("\n\n\n\n\nHello", str_left_pad("Hello", 10, '\n'));
        assert_eq!(".........ö", str_left_pad("ö", 10, '.'));
        assert_eq!(".........💋", str_left_pad("💋", 10, '.'));

        assert_eq!("x ", str_left_pad("x ", 2, '.'));
        assert_eq!("x ------", str_left_pad("x ------", 2, '.'));
        assert_eq!("ö ", str_left_pad("ö ", 2, '.'));
        assert_eq!("ö ------", str_left_pad("ö ------", 2, '.'));
        assert_eq!("💋 ", str_left_pad("💋 ", 2, '.'));
        assert_eq!("💋 ------", str_left_pad("💋 ------", 2, '.'));
    }
}
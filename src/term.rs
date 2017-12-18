#[allow(unused)]
pub fn hide_cursor() {
    print!("{}[?25l", 27 as char);
}

#[allow(unused)]
pub fn show_cursor() {
    print!("{}[?25h", 27 as char);
}

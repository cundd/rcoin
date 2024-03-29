use crate::term_style;

pub fn _error(message: &str) -> ! {
    term_style::cursor::show_cursor();

    #[cfg(debug_assertions)]
        panic!("{}", message);

    #[cfg(not(debug_assertions))]
        {
            eprintln!("{}", message);
            ::std::process::exit(1);
        }
}

macro_rules! error {
($($arg:tt)*) => ($crate::error::_error(&$crate::std::fmt::format(format_args!($($arg)*))));
}

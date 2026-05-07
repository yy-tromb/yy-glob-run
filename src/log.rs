#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("\n\x1b[1minfo: {}\x1b[0m\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        eprintln!("\n\x1b[1;32mok: {}\x1b[0m\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("\n\x1b[1;33mwarn: {}\x1b[0m\n", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("\n\x1b[1;31merror: {}\x1b[0m\n", format_args!($($arg)*));
    };
}

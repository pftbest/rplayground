use core::fmt;
use stdio::StdOut;
use rdi;

pub fn _print(args: fmt::Arguments) {
    let mut out = StdOut::new();
    fmt::write(&mut out, args).expect("Error writing to StdOut");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: fmt::Arguments, file: &'static str, line: u32) -> ! {
    _print(fmt);
    println!("file: {}; line: {}", file, line);
    rdi::swi_exit(0);
    loop {}
}

use core::fmt;

pub use cortex_m_semihosting::io::write_fmt as write_fmt;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt(fmt: fmt::Arguments, file: &'static str, line: u32) -> ! {
    write_fmt(fmt);
    hprintln!("\nfile: {}; line: {}", file, line);
    loop {}
}

#![no_std]
#![no_main]

#[macro_use]
extern crate rstm;

#[no_mangle]
pub extern fn main() -> ! {
    loop {
        let y = 0.1 + 0.2;
        println!("The value is {}", y);
        panic!("FUUU");
    }
}

#![no_std]
#![no_main]

#[macro_use]
extern crate rstm;
#[macro_use]
extern crate bitflags;

bitflags! {
    flags Flags: u32 {
        const FLAG_A       = 0b00000001,
        const FLAG_B       = 0b00000010,
        const FLAG_C       = 0b00000100,
        const FLAG_ABC     = FLAG_A.bits
                           | FLAG_B.bits
                           | FLAG_C.bits,
    }
}

#[no_mangle]
pub extern fn main() -> ! {
    loop {
        let x = ["Hello", " ", "World", "!"];
        println!("The program \"{}\" calculates the value {}", x[0], x[2]);
        panic!("UUU");
    }
}

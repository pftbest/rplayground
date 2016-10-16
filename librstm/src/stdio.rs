use rdi;
use core::fmt;

pub struct StdOut {
    fd: u32,
}

impl StdOut {
    pub fn get() -> Self {
        unsafe {
            static mut STDOUT: u32 = !0;
            if STDOUT == !0 {
                STDOUT = rdi::swi_open(":tt\0", rdi::FileMode::Write)
            }
            StdOut { fd: STDOUT }
        }
    }
}

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ptr = s.as_ptr();
        rdi::swi_write(self.fd, ptr, s.len());
        Ok(())
    }
}

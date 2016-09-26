use rdi;
use core::fmt;

pub struct StdOut {
    fd: u32,
}

impl StdOut {
    pub fn new() -> Self {
        StdOut { fd: rdi::swi_open(":tt\0", rdi::FileMode::Write) }
    }
}

impl Drop for StdOut {
    fn drop(&mut self) {
        rdi::swi_close(self.fd);
    }
}

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ptr = s.as_ptr();
        rdi::swi_write(self.fd, ptr, s.len());
        Ok(())
    }
}

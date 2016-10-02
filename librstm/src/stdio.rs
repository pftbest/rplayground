use rdi;
use core::fmt;
use core::cell::Cell;

struct LazyCell<T: Copy + Clone> {
    value: Cell<Option<T>>,
    generator: fn() -> T,
}
unsafe impl<T: Copy + Clone> Sync for LazyCell<T> {}

impl<T: Copy + Clone> LazyCell<T> {
    pub const fn new(gen: fn() -> T) -> Self {
        LazyCell {
            value: Cell::new(None),
            generator: gen
        }
    }

    pub fn get(&self) -> T {
        let v = self.value.get();
        match v {
            Some(x) => {x}
            None => {
                let f = self.generator;
                let x = f();
                self.value.set(Some(x));
                x
            }
        }
    }
}

fn open_stdout() -> u32 {
    rdi::swi_open(":tt\0", rdi::FileMode::Write)
}

static STDOUT: LazyCell<u32> = LazyCell::new(open_stdout);

pub struct StdOut {
    fd: u32,
}

impl StdOut {
    pub fn get() -> Self {
        StdOut { fd: STDOUT.get() }
    }
}

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ptr = s.as_ptr();
        rdi::swi_write(self.fd, ptr, s.len());
        Ok(())
    }
}

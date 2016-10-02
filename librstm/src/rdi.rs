#[allow(dead_code)]
enum SwiReason {
    Open = 0x01,
    Close = 0x02,
    WriteC = 0x03,
    Write0 = 0x04,
    Write = 0x05,
    Read = 0x06,
    ReadC = 0x07,
    IsTTY = 0x09,
    Seek = 0x0A,
    FLen = 0x0C,
    TmpNam = 0x0D,
    Remove = 0x0E,
    Rename = 0x0F,
    Clock = 0x10,
    Time = 0x11,
    System = 0x12,
    Errno = 0x13,
    GetCmdLine = 0x15,
    HeapInfo = 0x16,
    EnterSVC = 0x17,
    Exit = 0x18,
}

#[allow(dead_code)]
pub enum FileMode {
    Read = 0,
    Write = 4,
    Append = 8,
}

#[inline(never)]
fn angel_swi(reason: SwiReason, argument: *const u32) -> u32 {
    let result: u32;
    unsafe {
        asm! {
        "bkpt 0xab"
        : "={r0}" (result)
        : "{r0}" (reason as u32), "{r1}" (argument)
        : "r4", "memory"
        : "volatile"
    }
    }
    return result;
}

pub fn swi_open(name: &str, mode: FileMode) -> u32 {
    let ptr = name.as_ptr();
    let arg: [u32; 3] = [ptr as u32, name.len() as u32, mode as u32];
    return angel_swi(SwiReason::Open, arg.as_ptr());
}

#[allow(dead_code)]
pub fn swi_close(fd: u32) -> u32 {
    return angel_swi(SwiReason::Close, &fd);
}

pub fn swi_write(fd: u32, data: *const u8, count: usize) -> u32 {
    let arg: [u32; 3] = [fd, data as u32, count as u32];
    return angel_swi(SwiReason::Write, arg.as_ptr());
}

pub fn swi_exit(code: u32) {
    angel_swi(SwiReason::Exit, &code);
}

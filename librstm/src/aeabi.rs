use core::ptr;

#[no_mangle]
pub unsafe extern fn __aeabi_memclr4(mut dst: *mut u32, size: usize) {
    for _ in 0..(size / 4) {
        ptr::write_volatile(dst, 0);
        dst = dst.offset(1);
    }
    let mut d8 = dst as *mut u8;
    for _ in 0..(size % 4) {
        ptr::write_volatile(d8, 0);
        d8 = d8.offset(1);
    }
}

#[no_mangle]
pub unsafe extern fn __aeabi_memcpy4(mut dst: *mut u32, mut src: *const u32, size: usize) {
    for _ in 0..(size / 4) {
        ptr::write_volatile(dst, *src);
        src = src.offset(1);
        dst = dst.offset(1);
    }
    let mut s8 = src as *const u8;
    let mut d8 = dst as *mut u8;
    for _ in 0..(size % 4) {
        ptr::write_volatile(d8, *s8);
        s8 = s8.offset(1);
        d8 = d8.offset(1);
    }
}

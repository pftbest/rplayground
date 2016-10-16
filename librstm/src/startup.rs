use core::ptr;

#[no_mangle]
pub unsafe extern fn reset_handler() {
    init_data();
    init_bss();

    extern {
        fn main() -> !;
    }
    main();
}

unsafe fn init_data() {
    extern {
        static __data_load: u32;
        static mut __data_start: u32;
        static __data_end: u32;
    }
    let src = &__data_load as *const u32;
    let dst = &mut __data_start as *mut u32;
    let end = &__data_end as *const u32;
    let count = (end as usize) - (dst as usize);
    ptr::copy_nonoverlapping(src, dst, count);
}

unsafe fn init_bss() {
    extern {
        static mut __bss_start: u32;
        static __bss_end: u32;
    }
    let dst = &mut __bss_start as *mut u32;
    let end = &__bss_end as *const u32;
    let count = (end as usize) - (dst as usize);
    ptr::write_bytes(dst, 0, count);
}

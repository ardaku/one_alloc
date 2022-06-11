#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(default_alloc_error_handler)]
#![feature(core_c_str)]

extern crate alloc;

use core::ffi::CStr;

mod log {
    use core::ffi::CStr;

    #[link(name = "c")]
    extern "C" {
        fn puts(s: *const ()) -> i32;
    }

    pub fn println(text: &CStr) {
        unsafe {
            puts(text.as_ptr().cast());
        }
    }
}

mod main {
    use core::{ffi::CStr, mem::size_of};

    use one_alloc::Allocator;

    use super::log;

    #[global_allocator]
    static ALLOCATOR: Allocator<{ size_of::<usize>() * 2 }> = Allocator::new();

    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_panic: &PanicInfo<'_>) -> ! {
        log::println(CStr::from_bytes_with_nul(b"panicked!\0").unwrap());
        loop {}
    }

    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}

    #[no_mangle]
    extern "C" fn main() -> ! {
        loop {
            super::main();
            log::println(
                CStr::from_bytes_with_nul(b"WARNING: main is restarting!\0")
                    .unwrap(),
            );
        }
    }
}

fn main() {
    log::println(CStr::from_bytes_with_nul(b"Allocating...\0").unwrap());
    let _my_allocation = alloc::sync::Arc::new(());
    log::println(CStr::from_bytes_with_nul(b"Allocated!\0").unwrap());
}

#![feature(libc)]
#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![no_main]

extern crate libc;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    unsafe {
        libc::getpid()
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
#[lang = "eh_unwind_resume"] extern fn rust_eh_unwind_resume() {}
#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}
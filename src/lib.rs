#![no_std]
#![feature(lang_items)]
#![feature(const_format_args)]

pub fn cipher<T>(data: T) -> T { data }

pub fn decipher<T>(data: T) -> T { data }

#[allow(non_camel_case_types)]
type c_str = *const i8;

#[no_mangle]
extern "C" fn rot26_cipher(data: c_str) -> c_str {
    cipher(data)
}

#[no_mangle]
extern "C" fn rot26_decipher(data: c_str) -> c_str {
    decipher(data)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    extern "C" { fn exit(status: i32); }
    unsafe { exit(-1) }
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

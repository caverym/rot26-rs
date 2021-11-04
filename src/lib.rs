#![no_std]
#![feature(lang_items)]
#![feature(const_format_args)]

/// # Cipher
///
/// This is the cipher function for the ROT-26 algorithm.
/// It will take any type and apply the algorithm to it.
pub fn cipher<T>(data: T) -> T { data }

/// # Decipher
///
/// This is the decipher function for the ROT-26 algorithm.
/// It will take any type and decipher it from ROT-26 ciphertext
pub fn decipher<T>(data: T) -> T { data }

/// Defines a type to manage C's const char* type
#[allow(non_camel_case_types)]
type c_str = *const i8;

/// ROT-26 cipher function in the C interface.
#[no_mangle]
extern "C" fn rot26_cipher(data: c_str) -> c_str {
    cipher(data)
}

/// ROT-26 decipher function in the C interface.
#[no_mangle]
extern "C" fn rot26_decipher(data: c_str) -> c_str {
    decipher(data)
}

#[doc(hidden)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    extern "C" { fn exit(status: i32); }
    unsafe { exit(-1) }
    loop {}
}

#[doc(hidden)]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

extern crate test_alloc;

pub mod email;

use email::{Email};
use std::ptr;
use std::ffi::CString;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn email_parse<'a>(length: i32, line: *mut c_char) -> *mut Email {
    let c_str_line = unsafe { CString::from_raw(line) };

    if let Ok(s) = c_str_line.into_string() {
        match Email::from_string(s) {
            Ok(email) => Box::into_raw(Box::new(email)),
            Err(_) => ptr::null_mut()
        }
    }
    else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn email_display(p: &Email) {
    println!("{}", p);
}

#[no_mangle]
pub extern "C" fn email_free(email: *mut Email) {
    drop(email);
}


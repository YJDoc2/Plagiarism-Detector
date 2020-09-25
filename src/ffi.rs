use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

pub const EOL: i32 = 256;
pub const EOF: i32 = -1;

extern "C" {
    static yylineno: c_int;
    static tok_val: *mut c_char;
    static reference: *mut c_char;
    static mut filename: *mut c_char;
    fn tty_open();
    fn yylex() -> c_int;
    fn reset_val();
}

pub fn open_file(name: &str) {
    let path = CString::new(name).unwrap();
    unsafe {
        filename = path.as_ptr() as *mut i8;
        tty_open();
    }
}

pub fn safe_yylex() -> Option<i32> {
    let ret: i32;
    unsafe {
        ret = yylex();
    }
    return if ret == EOF { None } else { Some(ret) };
}

pub fn get_val() -> String {
    unsafe {
        if tok_val.is_null() {
            return String::from("");
        }
        return match CStr::from_ptr(tok_val).to_str() {
            Ok(s) => {
                let ret = s.to_owned();
                reset_val();
                ret
            }
            Err(_) => String::from(""),
        };
    }
}

pub fn get_ref() -> String {
    unsafe {
        if reference.is_null() {
            return String::from("");
        }
        return match CStr::from_ptr(reference).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => String::from(""),
        };
    }
}

pub fn get_lineno() -> i32 {
    unsafe { yylineno }
}

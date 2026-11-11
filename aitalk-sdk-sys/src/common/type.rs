use std::ffi::{c_char, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub union AITalkMixedType {
    pub int32: i32,
    pub floating: f64,
    pub str: *const c_char,
    pub any: *mut c_void,
    pub boolean: u8,
}

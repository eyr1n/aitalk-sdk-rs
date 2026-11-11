use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::{io, ptr};

use windows_sys::Win32::Globalization::{
    CP_ACP, CP_UTF8, GetACP, WC_NO_BEST_FIT_CHARS, WideCharToMultiByte,
};

use crate::error::{Error, Result};

pub(crate) fn to_ansi(string: &OsStr) -> Result<CString> {
    let wide: Vec<u16> = string.encode_wide().collect();
    if wide.is_empty() {
        return Ok(CString::default());
    }
    let wide_len = i32::try_from(wide.len())?;

    let mut used_default = 0;
    let (flags, used_default_ptr) = if unsafe { GetACP() } == CP_UTF8 {
        (0, ptr::null_mut())
    } else {
        (WC_NO_BEST_FIT_CHARS, &raw mut used_default)
    };

    let len = unsafe {
        WideCharToMultiByte(
            CP_ACP,
            flags,
            wide.as_ptr(),
            wide_len,
            ptr::null_mut(),
            0,
            ptr::null(),
            used_default_ptr,
        )
    };
    if len == 0 {
        return Err(io::Error::last_os_error().into());
    }

    let mut ansi = vec![0u8; usize::try_from(len)?];
    let len = unsafe {
        WideCharToMultiByte(
            CP_ACP,
            flags,
            wide.as_ptr(),
            wide_len,
            ansi.as_mut_ptr(),
            len,
            ptr::null(),
            used_default_ptr,
        )
    };
    if len == 0 {
        return Err(io::Error::last_os_error().into());
    }
    if used_default != 0 {
        return Err(Error::NotAnsi(string.to_owned()));
    }
    Ok(CString::new(ansi)?)
}

use std::ffi::CString;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

use aitalk_sdk_sys::AiTalkSdk;
use once_cell::sync::OnceCell;
use windows_sys::Win32::System::LibraryLoader::{GetDllDirectoryW, SetDllDirectoryW};

use crate::encoding::to_ansi;
use crate::error::{Result, to_result};

static ENGINE: OnceCell<Engine> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct EngineLicense {
    pub path: PathBuf,
    pub code: String,
    pub values: Vec<(i32, String)>,
}

pub struct Engine {
    pub(crate) sdk: AiTalkSdk,
}

impl Engine {
    pub fn init_once(
        sdk: AiTalkSdk,
        nlp_dll_dir: impl AsRef<Path>,
        license: EngineLicense,
    ) -> Result<&'static Self> {
        ENGINE.get_or_try_init(|| {
            let nlp_dll_dir: Vec<u16> = nlp_dll_dir
                .as_ref()
                .as_os_str()
                .encode_wide()
                .chain([0])
                .collect();
            let dll_dir = dll_directory();

            set_dll_directory(nlp_dll_dir.as_ptr())?;
            let initialized = to_result(unsafe { (sdk.AITalk_Core_initialize)() });
            let _ = if let Some(dir) = dll_dir {
                set_dll_directory(dir.as_ptr())
            } else {
                set_dll_directory(ptr::null())
            };
            initialized?;

            if let Err(e) = authenticate(&sdk, license) {
                unsafe { (sdk.AITalk_Core_finalize)() };
                return Err(e);
            }
            Ok(Self { sdk })
        })
    }
}

fn dll_directory() -> Option<Vec<u16>> {
    let len = unsafe { GetDllDirectoryW(0, ptr::null_mut()) };
    if len == 0 {
        return None;
    }
    let mut buf = vec![0u16; len as usize];
    if unsafe { GetDllDirectoryW(len, buf.as_mut_ptr()) } == 0 {
        return None;
    }
    Some(buf)
}

fn set_dll_directory(ptr: *const u16) -> Result<()> {
    if unsafe { SetDllDirectoryW(ptr) } == 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn authenticate(sdk: &AiTalkSdk, license: EngineLicense) -> Result<()> {
    let path = to_ansi(license.path.as_os_str())?;
    let code = CString::new(license.code)?;

    to_result(unsafe { (sdk.AITalk_Core_EngineLicense_setPath)(path.as_ptr()) })?;
    to_result(unsafe { (sdk.AITalk_Core_EngineLicense_setCode)(code.as_ptr()) })?;

    for (id, value) in license.values {
        let value = CString::new(value)?;
        to_result(unsafe { (sdk.AITalk_Core_EngineLicense_setValue)(id, value.as_ptr()) })?;
    }

    to_result(unsafe { (sdk.AITalk_Core_authenticate)() })?;
    Ok(())
}

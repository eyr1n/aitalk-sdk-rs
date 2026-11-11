use std::path::Path;
use std::ptr::{self, NonNull};

use aitalk_sdk_sys::AITalk_Core_VoiceDictionaryMetaContainer;

use crate::encoding::to_ansi;
use crate::engine::Engine;
use crate::error::{Result, to_result};

pub struct VoiceDictionaryMetaContainer {
    engine: &'static Engine,
    ptr: NonNull<AITalk_Core_VoiceDictionaryMetaContainer>,
}

impl VoiceDictionaryMetaContainer {
    pub fn new(engine: &'static Engine, path: &Path) -> Result<Self> {
        let mut ptr = ptr::null_mut();
        let path = to_ansi(path.as_os_str())?;
        to_result(unsafe {
            (engine.sdk.AITalk_Core_VoiceDictionaryMetaContainer_new)(&mut ptr, path.as_ptr())
        })?;
        Ok(Self {
            engine,
            ptr: NonNull::new(ptr).unwrap(),
        })
    }

    pub fn rate(&self) -> Result<i32> {
        let mut rate = 0;
        to_result(unsafe {
            (self
                .engine
                .sdk
                .AITalk_Core_VoiceDictionaryMetaContainer_getRate)(
                self.ptr.as_ptr(), &mut rate
            )
        })?;
        Ok(rate)
    }
}

impl Drop for VoiceDictionaryMetaContainer {
    fn drop(&mut self) {
        unsafe {
            (self
                .engine
                .sdk
                .AITalk_Core_VoiceDictionaryMetaContainer_delete)(self.ptr.as_ptr())
        };
    }
}

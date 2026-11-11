use std::ffi::CString;
use std::ptr::{self, NonNull};

use aitalk_sdk_sys::{
    AITalk_Core_TtsParameter, AITalk_Core_TtsParameterId_Emphasis,
    AITalk_Core_TtsParameterId_Pitch, AITalk_Core_TtsParameterId_Rate,
    AITalk_Core_TtsParameterId_StyleColor, AITalk_Core_TtsParameterId_Volume,
    AITalk_Core_TtsParameterId_WaitTime_Eos, AITalk_Core_TtsParameterId_WaitTime_LongText,
    AITalk_Core_TtsParameterId_WaitTime_MiddleText, AITalkMixedType,
};

use crate::engine::Engine;
use crate::error::{Result, to_result};

pub struct TtsParameter {
    engine: &'static Engine,
    pub(crate) ptr: NonNull<AITalk_Core_TtsParameter>,
}

impl TtsParameter {
    pub fn new(engine: &'static Engine) -> Result<Self> {
        let mut ptr = ptr::null_mut();
        to_result(unsafe { (engine.sdk.AITalk_Core_TtsParameter_new)(&mut ptr) })?;
        Ok(Self {
            engine,
            ptr: NonNull::new(ptr).unwrap(),
        })
    }

    pub fn set_wait_time_middle_text(&mut self, milliseconds: i32) -> Result<()> {
        self.put_value(
            AITalk_Core_TtsParameterId_WaitTime_MiddleText,
            AITalkMixedType {
                int32: milliseconds,
            },
        )
    }

    pub fn set_wait_time_long_text(&mut self, milliseconds: i32) -> Result<()> {
        self.put_value(
            AITalk_Core_TtsParameterId_WaitTime_LongText,
            AITalkMixedType {
                int32: milliseconds,
            },
        )
    }

    pub fn set_wait_time_eos(&mut self, milliseconds: i32) -> Result<()> {
        self.put_value(
            AITalk_Core_TtsParameterId_WaitTime_Eos,
            AITalkMixedType {
                int32: milliseconds,
            },
        )
    }

    pub fn set_volume(&mut self, volume: f64) -> Result<()> {
        self.put_value(
            AITalk_Core_TtsParameterId_Volume,
            AITalkMixedType { floating: volume },
        )
    }

    pub fn set_rate(&mut self, rate: f64) -> Result<()> {
        self.put_value(
            AITalk_Core_TtsParameterId_Rate,
            AITalkMixedType { floating: rate },
        )
    }

    pub fn set_pitch(&mut self, pitch: f64) -> Result<()> {
        self.put_value(
            AITalk_Core_TtsParameterId_Pitch,
            AITalkMixedType { floating: pitch },
        )
    }

    pub fn set_emphasis(&mut self, emphasis: f64) -> Result<()> {
        self.put_value(
            AITalk_Core_TtsParameterId_Emphasis,
            AITalkMixedType { floating: emphasis },
        )
    }

    pub fn set_style_color(&mut self, style_color: &str) -> Result<()> {
        let style_color = CString::new(style_color)?;
        self.put_value(
            AITalk_Core_TtsParameterId_StyleColor,
            AITalkMixedType {
                str: style_color.as_ptr(),
            },
        )
    }

    fn put_value(&mut self, id: i32, value: AITalkMixedType) -> Result<()> {
        to_result(unsafe {
            (self.engine.sdk.AITalk_Core_TtsParameter_putKeyValue)(
                self.ptr.as_ptr(),
                id,
                ptr::null(),
                value,
            )
        })
    }
}

impl Drop for TtsParameter {
    fn drop(&mut self) {
        unsafe { (self.engine.sdk.AITalk_Core_TtsParameter_delete)(self.ptr.as_ptr()) };
    }
}

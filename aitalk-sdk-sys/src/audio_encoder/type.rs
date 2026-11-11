use std::ffi::{c_char, c_void};

use crate::{AITalk_AudioEncoder_EncoderEventId, AITalkReturnCode};

pub type AITalk_AudioEncoder_EncodedCallback = Option<
    unsafe extern "C" fn(
        userdata: *mut c_void,
        id: AITalk_AudioEncoder_EncoderEventId,
        body: *mut c_void,
    ) -> AITalkReturnCode,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AITalk_AudioEncoder_Output {
    pub audio_buffer: *mut c_char,
    pub audio_buffer_size: usize,
    pub audio_buffer_offset: usize,
}

use std::ffi::c_void;

use crate::AITalk_AudioEncoder_EncodedCallback;

#[repr(C)]
pub struct AITalk_AudioEncoder_ResamplingEncoder {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
#[repr(C)]
pub struct AITalk_AudioEncoder_ResamplingEncoderConfig {
    pub current_rate: usize,
    pub conversion_rate: usize,
    pub callback: AITalk_AudioEncoder_EncodedCallback,
    pub userdata: *mut c_void,
    pub callback_size: usize,
    pub range: usize,
    pub depth: usize,
}

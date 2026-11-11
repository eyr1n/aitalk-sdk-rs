use std::ffi::c_int;

pub const AITalk_AudioEncoder_AudioConfigId_depth: AITalk_AudioEncoder_AudioConfigIdEnum = 1;
pub const AITalk_AudioEncoder_AudioConfigId_freq: AITalk_AudioEncoder_AudioConfigIdEnum = 2;
pub const AITalk_AudioEncoder_AudioConfigId_channel: AITalk_AudioEncoder_AudioConfigIdEnum = 3;
pub const AITalk_AudioEncoder_AudioConfigId_format: AITalk_AudioEncoder_AudioConfigIdEnum = 4;

pub type AITalk_AudioEncoder_AudioConfigIdEnum = c_int;
pub type AITalk_AudioEncoder_AudioConfigId = i32;

#[repr(C)]
pub struct AITalk_AudioEncoder_AudioConfig {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

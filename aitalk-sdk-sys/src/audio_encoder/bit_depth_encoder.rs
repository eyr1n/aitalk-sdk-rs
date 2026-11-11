use std::ffi::c_int;

pub const AITalk_AudioEncoder_BitDepthEncodePattern_Int16ToUint8:
    AITalk_AudioEncoder_BitDepthEncodePatternEnum = 1;

pub type AITalk_AudioEncoder_BitDepthEncodePatternEnum = c_int;
pub type AITalk_AudioEncoder_BitDepthEncodePattern = i32;

#[repr(C)]
pub struct AITalk_AudioEncoder_BitDepthEncoder {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

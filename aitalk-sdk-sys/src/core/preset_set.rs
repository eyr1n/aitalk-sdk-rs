use std::ffi::c_int;

pub const AITalkPresetSetId_LanguageDictionary: AITalkPresetSetIdEnum = 1;
pub const AITalkPresetSetId_VoiceDictionary: AITalkPresetSetIdEnum = 2;
pub const AITalkPresetSetId_Uid: AITalkPresetSetIdEnum = 3;
pub const AITalkPresetSetId_TtsParameter: AITalkPresetSetIdEnum = 4;

pub type AITalkPresetSetIdEnum = c_int;
pub type AITalkPresetSetId = i32;

#[repr(C)]
pub struct AITalk_Core_PresetSet {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

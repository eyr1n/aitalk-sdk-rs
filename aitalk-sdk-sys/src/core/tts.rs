use std::ffi::{c_int, c_void};

use crate::{AITalk_Core_TtsOutEventId, AITalkReturnCode};

pub const AITalk_Core_TtsId_LanguageDictionary: AITalk_Core_TtsIdEnum = 1;
pub const AITalk_Core_TtsId_VoiceDictionary: AITalk_Core_TtsIdEnum = 2;
pub const AITalk_Core_TtsId_VoiceDictionaryLicense: AITalk_Core_TtsIdEnum = 3;
pub const AITalk_Core_TtsId_TtsParameter: AITalk_Core_TtsIdEnum = 4;
pub const AITalk_Core_TtsId_InputEncoding: AITalk_Core_TtsIdEnum = 5;
pub const AITalk_Core_TtsId_TypeOfInput: AITalk_Core_TtsIdEnum = 6;
pub const AITalk_Core_TtsId_TagTokenBegin: AITalk_Core_TtsIdEnum = 100;
pub const AITalk_Core_TtsId_TagTokenEnd: AITalk_Core_TtsIdEnum = 101;
pub const AITalk_Core_TtsId_AutoBookmarkMode: AITalk_Core_TtsIdEnum = 201;
pub const AITalk_Core_TtsId_AutoBookmarkMarkPrefix: AITalk_Core_TtsIdEnum = 202;
pub const AITalk_Core_TtsId_BookmarkMarkerMode: AITalk_Core_TtsIdEnum = 301;
pub const AITalk_Core_TtsId_LipsyncMarkerMode: AITalk_Core_TtsIdEnum = 302;
pub const AITalk_Core_TtsId_AccentMarkerMode: AITalk_Core_TtsIdEnum = 303;
pub const AITalk_Core_TtsId_PhraseMarkerMode: AITalk_Core_TtsIdEnum = 304;
pub const AITalk_Core_TtsId_PresetSet: AITalk_Core_TtsIdEnum = 401;
pub const AITalk_Core_TtsId_Uid: AITalk_Core_TtsIdEnum = 402;

pub type AITalk_Core_TtsIdEnum = c_int;
pub type AITalk_Core_TtsId = i32;

pub const AITalk_Core_Tts_AutoBookmarkModeId_None: AITalk_Core_Tts_AutoBookmarkModeIdEnum = 1;
pub const AITalk_Core_Tts_AutoBookmarkModeId_Sentence: AITalk_Core_Tts_AutoBookmarkModeIdEnum = 2;
pub const AITalk_Core_Tts_AutoBookmarkModeId_HighLight: AITalk_Core_Tts_AutoBookmarkModeIdEnum = 3;

pub type AITalk_Core_Tts_AutoBookmarkModeIdEnum = c_int;
pub type AITalk_Core_Tts_AutoBookmarkModeId = i32;

#[repr(C)]
pub struct AITalk_Core_Tts {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AITalk_Core_TtsOutCallback = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        event_id: AITalk_Core_TtsOutEventId,
        data: *mut c_void,
    ) -> AITalkReturnCode,
>;

pub const AITalk_Core_Tts_MergeProsodyLevel_1: AITalk_Core_Tts_MergeProsodyLevelEnum = 1;
pub const AITalk_Core_Tts_MergeProsodyLevel_2: AITalk_Core_Tts_MergeProsodyLevelEnum = 2;

pub type AITalk_Core_Tts_MergeProsodyLevelEnum = c_int;
pub type AITalk_Core_Tts_MergeProsodyLevel = i32;

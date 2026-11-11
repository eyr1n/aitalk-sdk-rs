use std::ffi::{c_char, c_int, c_void};

use crate::{AITalk_Core_TtsOutEventId, AITalkReturnCode};

pub type AITalk_Core_CallbackSelector_BufreqCallback = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        audio_buffer: *mut *mut c_char,
        audio_buffer_size: *mut usize,
        marker_buffer: *mut *mut c_char,
        marker_buffer_size: *mut usize,
    ) -> AITalkReturnCode,
>;
pub type AITalk_Core_CallbackSelector_BufdoneCallback = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        audio_buffer: *mut c_char,
        audio_buffer_size: usize,
        marker_buffer: *mut c_char,
        marker_buffer_size: usize,
    ) -> AITalkReturnCode,
>;
pub type AITalk_Core_CallbackSelector_BookmarkCallback = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        event_id: AITalk_Core_TtsOutEventId,
        audio_sample_pos: usize,
        input_text_pos: usize,
        bookmark_id: *const c_char,
        bookmark_id_size: usize,
    ) -> AITalkReturnCode,
>;
pub type AITalk_Core_CallbackSelector_LipsyncCallback = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        event_id: AITalk_Core_TtsOutEventId,
        audio_sample_pos: usize,
        phoneme: *const c_char,
        phoneme_size: usize,
    ) -> AITalkReturnCode,
>;
pub type AITalk_Core_CallbackSelector_AccentCallback = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        event_id: AITalk_Core_TtsOutEventId,
        audio_sample_pos: usize,
        accent_index: usize,
    ) -> AITalkReturnCode,
>;
pub type AITalk_Core_CallbackSelector_PhraseCallback = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        event_id: AITalk_Core_TtsOutEventId,
        audio_sample_pos: usize,
        phrase_index: usize,
    ) -> AITalkReturnCode,
>;

pub const AITalk_Core_CallbackSelector_CallbackId_Bufreq:
    AITalk_Core_CallbackSelector_CallbackIdEnum = 1;
pub const AITalk_Core_CallbackSelector_CallbackId_Bufdone:
    AITalk_Core_CallbackSelector_CallbackIdEnum = 2;
pub const AITalk_Core_CallbackSelector_CallbackId_Bookmark:
    AITalk_Core_CallbackSelector_CallbackIdEnum = 3;
pub const AITalk_Core_CallbackSelector_CallbackId_Lipsync:
    AITalk_Core_CallbackSelector_CallbackIdEnum = 4;
pub const AITalk_Core_CallbackSelector_CallbackId_Accent:
    AITalk_Core_CallbackSelector_CallbackIdEnum = 5;
pub const AITalk_Core_CallbackSelector_CallbackId_Phrase:
    AITalk_Core_CallbackSelector_CallbackIdEnum = 6;

pub type AITalk_Core_CallbackSelector_CallbackIdEnum = c_int;
pub type AITalk_Core_CallbackSelector_CallbackId = i32;

#[repr(C)]
pub struct AITalk_Core_CallbackSelector {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

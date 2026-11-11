use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AITalk_Core_TtsBufreqEventInfo {
    pub audio_buffer: *mut c_char,
    pub audio_buffer_size: usize,
    pub marker_buffer: *mut c_char,
    pub marker_buffer_size: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AITalk_Core_TtsBufdoneEventInfo {
    pub audio_buffer: *mut c_char,
    pub audio_buffer_size: usize,
    pub marker_buffer: *mut c_char,
    pub marker_buffer_size: usize,
    pub marker_buffer_count: usize,
}
#[repr(C)]
pub struct AITalk_Core_TtsMarkerEventInfo {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AITalk_Core_BookmarkMarker {
    pub audio_sample_pos: usize,
    pub input_text_pos: usize,
    pub bookmark_id: *const c_char,
    pub bookmark_id_size: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AITalk_Core_LipsyncMarker {
    pub audio_sample_pos: usize,
    pub phoneme: *const c_char,
    pub phoneme_size: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AITalk_Core_AccentMarker {
    pub audio_sample_pos: usize,
    pub accent_index: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AITalk_Core_PhraseMarker {
    pub audio_sample_pos: usize,
    pub phrase_index: usize,
}

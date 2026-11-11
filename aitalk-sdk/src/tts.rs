use std::ffi::{CString, c_void};
use std::path::Path;
use std::ptr::{self, NonNull};

use aitalk_sdk_sys::{
    AITalk_Core_Tts, AITalk_Core_TtsBufdoneEventInfo, AITalk_Core_TtsBufreqEventInfo,
    AITalk_Core_TtsId_InputEncoding, AITalk_Core_TtsId_LanguageDictionary,
    AITalk_Core_TtsId_TtsParameter, AITalk_Core_TtsId_VoiceDictionary,
    AITalk_Core_TtsId_VoiceDictionaryLicense, AITalk_Core_TtsOutEventId,
    AITalk_Core_TtsOutEventId_Bufdone, AITalk_Core_TtsOutEventId_Bufreq,
    AITalk_TextEncodingsId_Utf8, AITalkMixedType, AITalkReturnCode,
    AITalkReturnCode_CallbackSuccess,
};

use crate::encoding::to_ansi;
use crate::engine::Engine;
use crate::error::{Result, to_result};
use crate::tts_parameter::TtsParameter;

const AUDIO_BUFFER_SAMPLES: usize = 0x10000;
const MARKER_BUFFER_SIZE: usize = 0x10000;

pub struct Tts {
    engine: &'static Engine,
    ptr: NonNull<AITalk_Core_Tts>,
}

unsafe impl Send for Tts {}

impl Tts {
    pub fn new(engine: &'static Engine) -> Result<Self> {
        let mut ptr = ptr::null_mut();
        to_result(unsafe { (engine.sdk.AITalk_Core_Tts_new)(&mut ptr) })?;
        let tts = Self {
            engine,
            ptr: NonNull::new(ptr).unwrap(),
        };
        to_result(unsafe {
            (tts.engine.sdk.AITalk_Core_Tts_putKeyValue)(
                tts.ptr.as_ptr(),
                AITalk_Core_TtsId_InputEncoding,
                ptr::null(),
                AITalkMixedType {
                    int32: AITalk_TextEncodingsId_Utf8,
                },
            )
        })?;
        Ok(tts)
    }

    pub fn duplicate(&self) -> Result<Self> {
        let mut ptr = ptr::null_mut();
        to_result(unsafe {
            (self.engine.sdk.AITalk_Core_Tts_duplicate)(&mut ptr, self.ptr.as_ptr())
        })?;
        Ok(Self {
            engine: self.engine,
            ptr: NonNull::new(ptr).unwrap(),
        })
    }

    pub fn put_language_dictionary(&mut self, key: &str, path: &Path) -> Result<()> {
        let path = to_ansi(path.as_os_str())?;
        self.put_key_value(
            AITalk_Core_TtsId_LanguageDictionary,
            key,
            AITalkMixedType { str: path.as_ptr() },
        )
    }

    pub fn put_voice_dictionary(&mut self, key: &str, path: &Path) -> Result<()> {
        let path = to_ansi(path.as_os_str())?;
        self.put_key_value(
            AITalk_Core_TtsId_VoiceDictionary,
            key,
            AITalkMixedType { str: path.as_ptr() },
        )
    }

    pub fn put_voice_dictionary_license(&mut self, key: &str, path: &Path) -> Result<()> {
        let path = to_ansi(path.as_os_str())?;
        self.put_key_value(
            AITalk_Core_TtsId_VoiceDictionaryLicense,
            key,
            AITalkMixedType { str: path.as_ptr() },
        )
    }

    pub fn put_parameter(&mut self, key: &str, parameter: &TtsParameter) -> Result<()> {
        self.put_key_value(
            AITalk_Core_TtsId_TtsParameter,
            key,
            AITalkMixedType {
                any: parameter.ptr.as_ptr().cast(),
            },
        )
    }

    pub fn select_language_dictionary(&mut self, key: &str) -> Result<()> {
        self.select_default_key(AITalk_Core_TtsId_LanguageDictionary, key)
    }

    pub fn select_voice_dictionary(&mut self, key: &str) -> Result<()> {
        self.select_default_key(AITalk_Core_TtsId_VoiceDictionary, key)
    }

    pub fn select_parameter(&mut self, key: &str) -> Result<()> {
        self.select_default_key(AITalk_Core_TtsId_TtsParameter, key)
    }

    pub fn run(&mut self, input: &str) -> Result<Vec<i16>> {
        let input = CString::new(input)?;
        let mut output = Output {
            audio_buffer: vec![0; AUDIO_BUFFER_SAMPLES],
            marker_buffer: vec![0; MARKER_BUFFER_SIZE],
            pcm: Vec::new(),
        };
        to_result(unsafe {
            (self.engine.sdk.AITalk_Core_Tts_run)(
                self.ptr.as_ptr(),
                input.as_ptr(),
                Some(on_tts_event),
                (&raw mut output).cast(),
            )
        })?;
        Ok(output.pcm)
    }

    fn put_key_value(&mut self, id: i32, key: &str, value: AITalkMixedType) -> Result<()> {
        let key = CString::new(key)?;
        to_result(unsafe {
            (self.engine.sdk.AITalk_Core_Tts_putKeyValue)(
                self.ptr.as_ptr(),
                id,
                key.as_ptr(),
                value,
            )
        })
    }

    fn select_default_key(&mut self, id: i32, key: &str) -> Result<()> {
        let key = CString::new(key)?;
        to_result(unsafe {
            (self.engine.sdk.AITalk_Core_Tts_selectDefaultKey)(self.ptr.as_ptr(), id, key.as_ptr())
        })
    }
}

impl Drop for Tts {
    fn drop(&mut self) {
        unsafe { (self.engine.sdk.AITalk_Core_Tts_delete)(self.ptr.as_ptr()) };
    }
}

struct Output {
    audio_buffer: Vec<i16>,
    marker_buffer: Vec<u8>,
    pcm: Vec<i16>,
}

unsafe extern "C" fn on_tts_event(
    user_data: *mut c_void,
    event_id: AITalk_Core_TtsOutEventId,
    data: *mut c_void,
) -> AITalkReturnCode {
    let output = unsafe { &mut *user_data.cast::<Output>() };
    if event_id == AITalk_Core_TtsOutEventId_Bufreq {
        let info = unsafe { &mut *data.cast::<AITalk_Core_TtsBufreqEventInfo>() };
        info.audio_buffer = output.audio_buffer.as_mut_ptr().cast();
        info.audio_buffer_size = output.audio_buffer.len() * size_of::<i16>();
        info.marker_buffer = output.marker_buffer.as_mut_ptr().cast();
        info.marker_buffer_size = output.marker_buffer.len();
    } else if event_id == AITalk_Core_TtsOutEventId_Bufdone {
        let info = unsafe { &*data.cast::<AITalk_Core_TtsBufdoneEventInfo>() };
        let samples = info.audio_buffer_size / size_of::<i16>();
        output
            .pcm
            .extend_from_slice(&output.audio_buffer[..samples]);
    }
    AITalkReturnCode_CallbackSuccess
}

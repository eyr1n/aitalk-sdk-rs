#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc
)]

use std::ffi::{c_char, c_void};
use std::path::Path;

pub use libloading::Error;
use libloading::Library;
use libloading::os::windows::{LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR};

mod audio_encoder;
mod common;
mod core;

pub use crate::audio_encoder::*;
pub use crate::common::*;
pub use crate::core::*;

pub struct AiTalkSdk {
    pub AITalk_Core_AIKanaContainer_getAIKana: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_AIKanaContainer,
        text: *mut *const c_char,
        size: *mut usize,
    ) -> AITalkReturnCode,
    pub AITalk_Core_AIKanaContainer_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_Core_AIKanaContainer) -> AITalkReturnCode,
    pub AITalk_Core_CallbackSelector_new:
        unsafe extern "C" fn(ptrptr: *mut *mut AITalk_Core_CallbackSelector) -> AITalkReturnCode,
    pub AITalk_Core_CallbackSelector_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_Core_CallbackSelector) -> AITalkReturnCode,
    pub AITalk_Core_CallbackSelector_putValue: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_CallbackSelector,
        id: AITalk_Core_CallbackSelector_CallbackId,
        callback: *mut c_void,
    ) -> AITalkReturnCode,
    pub AITalk_Core_CallbackSelector_select: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_CallbackSelector,
        user_data: *mut c_void,
        event_id: AITalk_Core_TtsOutEventId,
        data: *mut c_void,
    ) -> AITalkReturnCode,
    pub AITalk_Core_resetGlobalConfig: unsafe extern "C" fn() -> AITalkReturnCode,
    pub AITalk_Core_setGlobalConfig:
        unsafe extern "C" fn(id: AITalkGlobalConfigId, value: AITalkMixedType) -> AITalkReturnCode,
    pub AITalk_Core_initialize: unsafe extern "C" fn() -> AITalkReturnCode,
    pub AITalk_Core_finalize: unsafe extern "C" fn() -> AITalkReturnCode,
    pub AITalk_Core_setLogLevel:
        unsafe extern "C" fn(value: AITalk_Core_GlobalConfig_Log_Level) -> AITalkReturnCode,
    pub AITalk_Core_LanguageDictionaryMetaContainer_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_Core_LanguageDictionaryMetaContainer,
        path: *const c_char,
    )
        -> AITalkReturnCode,
    pub AITalk_Core_LanguageDictionaryMetaContainer_delete:
        unsafe extern "C" fn(
            ptr: *mut AITalk_Core_LanguageDictionaryMetaContainer,
        ) -> AITalkReturnCode,
    pub AITalk_Core_LanguageDictionaryMetaContainer_getValue:
        unsafe extern "C" fn(
            ptr: *mut AITalk_Core_LanguageDictionaryMetaContainer,
            id: AITalk_Core_LanguageDictionaryMetaContainerId,
            value: *mut AITalkMixedType,
        ) -> AITalkReturnCode,
    pub AITalk_Core_LanguageDictionaryMetaContainer_getVersion:
        unsafe extern "C" fn(
            ptr: *mut AITalk_Core_LanguageDictionaryMetaContainer,
            version: *mut *const c_char,
        ) -> AITalkReturnCode,
    pub AITalk_Core_authenticate: unsafe extern "C" fn() -> AITalkReturnCode,
    pub AITalk_Core_deauthenticate: unsafe extern "C" fn() -> AITalkReturnCode,
    pub AITalk_Core_EngineLicense_setValue: unsafe extern "C" fn(
        id: AITalk_Core_EngineLicenseId,
        value: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_EngineLicense_setPath:
        unsafe extern "C" fn(path: *const c_char) -> AITalkReturnCode,
    pub AITalk_Core_EngineLicense_setCode:
        unsafe extern "C" fn(code: *const c_char) -> AITalkReturnCode,
    pub AITalk_Core_EngineLicense_getExpiration:
        unsafe extern "C" fn(year: *mut i32, month: *mut i32, day: *mut i32) -> AITalkReturnCode,
    pub AITalk_Core_EngineLicense_getExpirationYmd:
        unsafe extern "C" fn(ymd: *mut i32) -> AITalkReturnCode,
    pub AITalk_Core_Marker_getTtsMarkerEventInfo: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_Core_TtsMarkerEventInfo,
        buffer: *mut c_char,
        buffer_size: usize,
        index: usize,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Marker_getType: unsafe extern "C" fn(
        r#type: *mut AITalk_Core_MarkerTypeId,
        data: *mut AITalk_Core_TtsMarkerEventInfo,
    ) -> AITalkReturnCode,
    pub AITalk_Core_BookmarkMarker_getValues: unsafe extern "C" fn(
        marker: *mut AITalk_Core_BookmarkMarker,
        data: *mut AITalk_Core_TtsMarkerEventInfo,
    ) -> AITalkReturnCode,
    pub AITalk_Core_LipsyncMarker_getValues: unsafe extern "C" fn(
        marker: *mut AITalk_Core_LipsyncMarker,
        data: *mut AITalk_Core_TtsMarkerEventInfo,
    ) -> AITalkReturnCode,
    pub AITalk_Core_AccentMarker_getValues: unsafe extern "C" fn(
        marker: *mut AITalk_Core_AccentMarker,
        data: *mut AITalk_Core_TtsMarkerEventInfo,
    ) -> AITalkReturnCode,
    pub AITalk_Core_PhraseMarker_getValues: unsafe extern "C" fn(
        marker: *mut AITalk_Core_PhraseMarker,
        data: *mut AITalk_Core_TtsMarkerEventInfo,
    ) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_new:
        unsafe extern "C" fn(ptrptr: *mut *mut AITalk_Core_TtsParameter) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_Core_TtsParameter) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_duplicate: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_Core_TtsParameter,
        ptr: *mut AITalk_Core_TtsParameter,
    ) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_copy: unsafe extern "C" fn(
        dst_ptr: *mut AITalk_Core_TtsParameter,
        src_ptr: *mut AITalk_Core_TtsParameter,
    ) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_putKeyValue: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_TtsParameter,
        id: AITalk_Core_TtsParameterId,
        key: *const c_char,
        value: AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_getKeyValue: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_TtsParameter,
        id: AITalk_Core_TtsParameterId,
        key: *const c_char,
        value: *mut AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_getKeyList: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_TtsParameter,
        id: AITalk_Core_TtsParameterId,
        keyList: *mut *const *const c_char,
        size: *mut usize,
    ) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_hasKey: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_TtsParameter,
        id: AITalk_Core_TtsParameterId,
        key: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_TtsParameter_deleteKey: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_TtsParameter,
        id: AITalk_Core_TtsParameterId,
        key: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_PresetSet_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_Core_PresetSet,
        language_key: *const c_char,
        voice_key: *const c_char,
        uid: *const c_char,
        tts_parameter: *mut AITalk_Core_TtsParameter,
    ) -> AITalkReturnCode,
    pub AITalk_Core_PresetSet_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_Core_PresetSet) -> AITalkReturnCode,
    pub AITalk_Core_PresetSet_set: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_PresetSet,
        id: AITalkPresetSetId,
        value: AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_Core_PresetSet_get: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_PresetSet,
        id: AITalkPresetSetId,
        value: *mut AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_new:
        unsafe extern "C" fn(ptrptr: *mut *mut AITalk_Core_Tts) -> AITalkReturnCode,
    pub AITalk_Core_Tts_delete: unsafe extern "C" fn(ptr: *mut AITalk_Core_Tts) -> AITalkReturnCode,
    pub AITalk_Core_Tts_duplicate: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_Core_Tts,
        ptr: *mut AITalk_Core_Tts,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_copy: unsafe extern "C" fn(
        dst_ptr: *mut AITalk_Core_Tts,
        src_ptr: *mut AITalk_Core_Tts,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_putKeyValue: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        id: AITalk_Core_TtsId,
        key: *const c_char,
        value: AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_getKeyValue: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        id: AITalk_Core_TtsId,
        key: *const c_char,
        value: *mut AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_getKeyList: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        id: AITalk_Core_TtsId,
        keyList: *mut *const *const c_char,
        size: *mut usize,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_selectDefaultKey: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        id: AITalk_Core_TtsId,
        key: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_getDefaultKey: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        id: AITalk_Core_TtsId,
        key: *mut *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_hasKey: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        id: AITalk_Core_TtsId,
        key: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_deleteKey: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        id: AITalk_Core_TtsId,
        key: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_isBusy: unsafe extern "C" fn(ptr: *mut AITalk_Core_Tts) -> AITalkReturnCode,
    pub AITalk_Core_Tts_run: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        input: *const c_char,
        callback: AITalk_Core_TtsOutCallback,
        user_data: *mut c_void,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_generateAIKanaContainer: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        ptrptr: *mut *mut AITalk_Core_AIKanaContainer,
        text: *const c_char,
        encoding: AITalk_TextEncodingsId,
    ) -> AITalkReturnCode,
    pub AITalk_Core_Tts_generateMergedVoiceDictionary: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_Tts,
        new_key: *const c_char,
        base_key: *const c_char,
        additional_key: *const c_char,
        level: AITalk_Core_Tts_MergeProsodyLevel,
    )
        -> AITalkReturnCode,
    pub AITalk_Core_createBinaryUserWordDictionary: unsafe extern "C" fn(
        text_path: *const c_char,
        binary_path: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_normalizeUtf8: unsafe extern "C" fn(
        original_text: *const c_char,
        normalized_text: *mut c_char,
        size: *mut usize,
    ) -> AITalkReturnCode,
    pub AITalk_Core_getVersion:
        unsafe extern "C" fn(version: *mut *const c_char) -> AITalkReturnCode,
    pub AITalk_Core_VoiceDictionaryMetaContainer_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_Core_VoiceDictionaryMetaContainer,
        path: *const c_char,
    ) -> AITalkReturnCode,
    pub AITalk_Core_VoiceDictionaryMetaContainer_delete: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_VoiceDictionaryMetaContainer,
    )
        -> AITalkReturnCode,
    pub AITalk_Core_VoiceDictionaryMetaContainer_getValue: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_VoiceDictionaryMetaContainer,
        id: AITalk_Core_VoiceDictionaryMetaContainerId,
        value: *mut AITalkMixedType,
    )
        -> AITalkReturnCode,
    pub AITalk_Core_VoiceDictionaryMetaContainer_getName: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_VoiceDictionaryMetaContainer,
        name: *mut *const c_char,
    )
        -> AITalkReturnCode,
    pub AITalk_Core_VoiceDictionaryMetaContainer_getGender:
        unsafe extern "C" fn(
            ptr: *mut AITalk_Core_VoiceDictionaryMetaContainer,
            gender: *mut *const c_char,
        ) -> AITalkReturnCode,
    pub AITalk_Core_VoiceDictionaryMetaContainer_getRate: unsafe extern "C" fn(
        ptr: *mut AITalk_Core_VoiceDictionaryMetaContainer,
        rate: *mut i32,
    )
        -> AITalkReturnCode,
    pub AITalk_Core_VoiceDictionaryMetaContainer_getStyleColorLabels:
        unsafe extern "C" fn(
            ptr: *mut AITalk_Core_VoiceDictionaryMetaContainer,
            labels: *mut *const c_char,
        ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_AudioConfig_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_AudioEncoder_AudioConfig,
        depth: u16,
        freq: u32,
        channel: u16,
        format: AITalk_AudioEncoder_SoundFormatType,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_AudioConfig_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_AudioConfig) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_AudioConfig_setValue: unsafe extern "C" fn(
        ptr: *mut AITalk_AudioEncoder_AudioConfig,
        id: AITalk_AudioEncoder_AudioConfigId,
        value: AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_AudioConfig_getValue: unsafe extern "C" fn(
        ptr: *mut AITalk_AudioEncoder_AudioConfig,
        id: AITalk_AudioEncoder_AudioConfigId,
        value: *mut AITalkMixedType,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_BitDepthEncoder_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_AudioEncoder_BitDepthEncoder,
        encode_pattern: AITalk_AudioEncoder_BitDepthEncodePattern,
        callback: AITalk_AudioEncoder_EncodedCallback,
        userdata: *mut c_void,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_BitDepthEncoder_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_BitDepthEncoder) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_BitDepthEncoder_addData: unsafe extern "C" fn(
        ptr: *mut AITalk_AudioEncoder_BitDepthEncoder,
        buf: *const c_char,
        size: usize,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_BitDepthEncoder_endData:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_BitDepthEncoder) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_MulawEncoder_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_AudioEncoder_MulawEncoder,
        callback: AITalk_AudioEncoder_EncodedCallback,
        userdata: *mut c_void,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_MulawEncoder_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_MulawEncoder) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_MulawEncoder_addData: unsafe extern "C" fn(
        ptr: *mut AITalk_AudioEncoder_MulawEncoder,
        buf: *const c_char,
        size: usize,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_MulawEncoder_endData:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_MulawEncoder) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_ResamplingEncoderConfig_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_AudioEncoder_ResamplingEncoderConfig,
        current_rate: usize,
        conversion_rate: usize,
        callback: AITalk_AudioEncoder_EncodedCallback,
        userdata: *mut c_void,
    )
        -> AITalkReturnCode,
    pub AITalk_AudioEncoder_ResamplingEncoderConfig_delete:
        unsafe extern "C" fn(
            ptr: *mut AITalk_AudioEncoder_ResamplingEncoderConfig,
        ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_ResamplingEncoder_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_AudioEncoder_ResamplingEncoder,
        config: *const AITalk_AudioEncoder_ResamplingEncoderConfig,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_ResamplingEncoder_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_ResamplingEncoder) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_ResamplingEncoder_addData: unsafe extern "C" fn(
        ptr: *mut AITalk_AudioEncoder_ResamplingEncoder,
        buf: *const c_char,
        size: usize,
    )
        -> AITalkReturnCode,
    pub AITalk_AudioEncoder_ResamplingEncoder_endData:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_ResamplingEncoder) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_WaveEncoder_new: unsafe extern "C" fn(
        ptrptr: *mut *mut AITalk_AudioEncoder_WaveEncoder,
        config_ptr: *mut AITalk_AudioEncoder_AudioConfig,
        callback: AITalk_AudioEncoder_EncodedCallback,
        userdata: *mut c_void,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_WaveEncoder_delete:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_WaveEncoder) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_WaveEncoder_addData: unsafe extern "C" fn(
        ptr: *mut AITalk_AudioEncoder_WaveEncoder,
        buf: *const c_char,
        size: usize,
    ) -> AITalkReturnCode,
    pub AITalk_AudioEncoder_WaveEncoder_endData:
        unsafe extern "C" fn(ptr: *mut AITalk_AudioEncoder_WaveEncoder) -> AITalkReturnCode,

    __library: Library,
}

impl AiTalkSdk {
    pub unsafe fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let __library: Library = unsafe {
            libloading::os::windows::Library::load_with_flags(
                path.as_ref(),
                LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
            )
        }?
        .into();

        Ok(AiTalkSdk {
            AITalk_Core_AIKanaContainer_getAIKana: *unsafe {
                __library.get(b"AITalk_Core_AIKanaContainer_getAIKana\0")
            }?,
            AITalk_Core_AIKanaContainer_delete: *unsafe {
                __library.get(b"AITalk_Core_AIKanaContainer_delete\0")
            }?,
            AITalk_Core_CallbackSelector_new: *unsafe {
                __library.get(b"AITalk_Core_CallbackSelector_new\0")
            }?,
            AITalk_Core_CallbackSelector_delete: *unsafe {
                __library.get(b"AITalk_Core_CallbackSelector_delete\0")
            }?,
            AITalk_Core_CallbackSelector_putValue: *unsafe {
                __library.get(b"AITalk_Core_CallbackSelector_putValue\0")
            }?,
            AITalk_Core_CallbackSelector_select: *unsafe {
                __library.get(b"AITalk_Core_CallbackSelector_select\0")
            }?,
            AITalk_Core_resetGlobalConfig: *unsafe {
                __library.get(b"AITalk_Core_resetGlobalConfig\0")
            }?,
            AITalk_Core_setGlobalConfig: *unsafe {
                __library.get(b"AITalk_Core_setGlobalConfig\0")
            }?,
            AITalk_Core_initialize: *unsafe { __library.get(b"AITalk_Core_initialize\0") }?,
            AITalk_Core_finalize: *unsafe { __library.get(b"AITalk_Core_finalize\0") }?,
            AITalk_Core_setLogLevel: *unsafe { __library.get(b"AITalk_Core_setLogLevel\0") }?,
            AITalk_Core_LanguageDictionaryMetaContainer_new: *unsafe {
                __library.get(b"AITalk_Core_LanguageDictionaryMetaContainer_new\0")
            }?,
            AITalk_Core_LanguageDictionaryMetaContainer_delete: *unsafe {
                __library.get(b"AITalk_Core_LanguageDictionaryMetaContainer_delete\0")
            }?,
            AITalk_Core_LanguageDictionaryMetaContainer_getValue: *unsafe {
                __library.get(b"AITalk_Core_LanguageDictionaryMetaContainer_getValue\0")
            }?,
            AITalk_Core_LanguageDictionaryMetaContainer_getVersion: *unsafe {
                __library.get(b"AITalk_Core_LanguageDictionaryMetaContainer_getVersion\0")
            }?,
            AITalk_Core_authenticate: *unsafe { __library.get(b"AITalk_Core_authenticate\0") }?,
            AITalk_Core_deauthenticate: *unsafe { __library.get(b"AITalk_Core_deauthenticate\0") }?,
            AITalk_Core_EngineLicense_setValue: *unsafe {
                __library.get(b"AITalk_Core_EngineLicense_setValue\0")
            }?,
            AITalk_Core_EngineLicense_setPath: *unsafe {
                __library.get(b"AITalk_Core_EngineLicense_setPath\0")
            }?,
            AITalk_Core_EngineLicense_setCode: *unsafe {
                __library.get(b"AITalk_Core_EngineLicense_setCode\0")
            }?,
            AITalk_Core_EngineLicense_getExpiration: *unsafe {
                __library.get(b"AITalk_Core_EngineLicense_getExpiration\0")
            }?,
            AITalk_Core_EngineLicense_getExpirationYmd: *unsafe {
                __library.get(b"AITalk_Core_EngineLicense_getExpirationYmd\0")
            }?,
            AITalk_Core_Marker_getTtsMarkerEventInfo: *unsafe {
                __library.get(b"AITalk_Core_Marker_getTtsMarkerEventInfo\0")
            }?,
            AITalk_Core_Marker_getType: *unsafe { __library.get(b"AITalk_Core_Marker_getType\0") }?,
            AITalk_Core_BookmarkMarker_getValues: *unsafe {
                __library.get(b"AITalk_Core_BookmarkMarker_getValues\0")
            }?,
            AITalk_Core_LipsyncMarker_getValues: *unsafe {
                __library.get(b"AITalk_Core_LipsyncMarker_getValues\0")
            }?,
            AITalk_Core_AccentMarker_getValues: *unsafe {
                __library.get(b"AITalk_Core_AccentMarker_getValues\0")
            }?,
            AITalk_Core_PhraseMarker_getValues: *unsafe {
                __library.get(b"AITalk_Core_PhraseMarker_getValues\0")
            }?,
            AITalk_Core_TtsParameter_new: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_new\0")
            }?,
            AITalk_Core_TtsParameter_delete: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_delete\0")
            }?,
            AITalk_Core_TtsParameter_duplicate: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_duplicate\0")
            }?,
            AITalk_Core_TtsParameter_copy: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_copy\0")
            }?,
            AITalk_Core_TtsParameter_putKeyValue: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_putKeyValue\0")
            }?,
            AITalk_Core_TtsParameter_getKeyValue: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_getKeyValue\0")
            }?,
            AITalk_Core_TtsParameter_getKeyList: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_getKeyList\0")
            }?,
            AITalk_Core_TtsParameter_hasKey: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_hasKey\0")
            }?,
            AITalk_Core_TtsParameter_deleteKey: *unsafe {
                __library.get(b"AITalk_Core_TtsParameter_deleteKey\0")
            }?,
            AITalk_Core_PresetSet_new: *unsafe { __library.get(b"AITalk_Core_PresetSet_new\0") }?,
            AITalk_Core_PresetSet_delete: *unsafe {
                __library.get(b"AITalk_Core_PresetSet_delete\0")
            }?,
            AITalk_Core_PresetSet_set: *unsafe { __library.get(b"AITalk_Core_PresetSet_set\0") }?,
            AITalk_Core_PresetSet_get: *unsafe { __library.get(b"AITalk_Core_PresetSet_get\0") }?,
            AITalk_Core_Tts_new: *unsafe { __library.get(b"AITalk_Core_Tts_new\0") }?,
            AITalk_Core_Tts_delete: *unsafe { __library.get(b"AITalk_Core_Tts_delete\0") }?,
            AITalk_Core_Tts_duplicate: *unsafe { __library.get(b"AITalk_Core_Tts_duplicate\0") }?,
            AITalk_Core_Tts_copy: *unsafe { __library.get(b"AITalk_Core_Tts_copy\0") }?,
            AITalk_Core_Tts_putKeyValue: *unsafe {
                __library.get(b"AITalk_Core_Tts_putKeyValue\0")
            }?,
            AITalk_Core_Tts_getKeyValue: *unsafe {
                __library.get(b"AITalk_Core_Tts_getKeyValue\0")
            }?,
            AITalk_Core_Tts_getKeyList: *unsafe { __library.get(b"AITalk_Core_Tts_getKeyList\0") }?,
            AITalk_Core_Tts_selectDefaultKey: *unsafe {
                __library.get(b"AITalk_Core_Tts_selectDefaultKey\0")
            }?,
            AITalk_Core_Tts_getDefaultKey: *unsafe {
                __library.get(b"AITalk_Core_Tts_getDefaultKey\0")
            }?,
            AITalk_Core_Tts_hasKey: *unsafe { __library.get(b"AITalk_Core_Tts_hasKey\0") }?,
            AITalk_Core_Tts_deleteKey: *unsafe { __library.get(b"AITalk_Core_Tts_deleteKey\0") }?,
            AITalk_Core_Tts_isBusy: *unsafe { __library.get(b"AITalk_Core_Tts_isBusy\0") }?,
            AITalk_Core_Tts_run: *unsafe { __library.get(b"AITalk_Core_Tts_run\0") }?,
            AITalk_Core_Tts_generateAIKanaContainer: *unsafe {
                __library.get(b"AITalk_Core_Tts_generateAIKanaContainer\0")
            }?,
            AITalk_Core_Tts_generateMergedVoiceDictionary: *unsafe {
                __library.get(b"AITalk_Core_Tts_generateMergedVoiceDictionary\0")
            }?,
            AITalk_Core_createBinaryUserWordDictionary: *unsafe {
                __library.get(b"AITalk_Core_createBinaryUserWordDictionary\0")
            }?,
            AITalk_Core_normalizeUtf8: *unsafe { __library.get(b"AITalk_Core_normalizeUtf8\0") }?,
            AITalk_Core_getVersion: *unsafe { __library.get(b"AITalk_Core_getVersion\0") }?,
            AITalk_Core_VoiceDictionaryMetaContainer_new: *unsafe {
                __library.get(b"AITalk_Core_VoiceDictionaryMetaContainer_new\0")
            }?,
            AITalk_Core_VoiceDictionaryMetaContainer_delete: *unsafe {
                __library.get(b"AITalk_Core_VoiceDictionaryMetaContainer_delete\0")
            }?,
            AITalk_Core_VoiceDictionaryMetaContainer_getValue: *unsafe {
                __library.get(b"AITalk_Core_VoiceDictionaryMetaContainer_getValue\0")
            }?,
            AITalk_Core_VoiceDictionaryMetaContainer_getName: *unsafe {
                __library.get(b"AITalk_Core_VoiceDictionaryMetaContainer_getName\0")
            }?,
            AITalk_Core_VoiceDictionaryMetaContainer_getGender: *unsafe {
                __library.get(b"AITalk_Core_VoiceDictionaryMetaContainer_getGender\0")
            }?,
            AITalk_Core_VoiceDictionaryMetaContainer_getRate: *unsafe {
                __library.get(b"AITalk_Core_VoiceDictionaryMetaContainer_getRate\0")
            }?,
            AITalk_Core_VoiceDictionaryMetaContainer_getStyleColorLabels: *unsafe {
                __library.get(b"AITalk_Core_VoiceDictionaryMetaContainer_getStyleColorLabels\0")
            }?,
            AITalk_AudioEncoder_AudioConfig_new: *unsafe {
                __library.get(b"AITalk_AudioEncoder_AudioConfig_new\0")
            }?,
            AITalk_AudioEncoder_AudioConfig_delete: *unsafe {
                __library.get(b"AITalk_AudioEncoder_AudioConfig_delete\0")
            }?,
            AITalk_AudioEncoder_AudioConfig_setValue: *unsafe {
                __library.get(b"AITalk_AudioEncoder_AudioConfig_setValue\0")
            }?,
            AITalk_AudioEncoder_AudioConfig_getValue: *unsafe {
                __library.get(b"AITalk_AudioEncoder_AudioConfig_getValue\0")
            }?,
            AITalk_AudioEncoder_BitDepthEncoder_new: *unsafe {
                __library.get(b"AITalk_AudioEncoder_BitDepthEncoder_new\0")
            }?,
            AITalk_AudioEncoder_BitDepthEncoder_delete: *unsafe {
                __library.get(b"AITalk_AudioEncoder_BitDepthEncoder_delete\0")
            }?,
            AITalk_AudioEncoder_BitDepthEncoder_addData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_BitDepthEncoder_addData\0")
            }?,
            AITalk_AudioEncoder_BitDepthEncoder_endData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_BitDepthEncoder_endData\0")
            }?,
            AITalk_AudioEncoder_MulawEncoder_new: *unsafe {
                __library.get(b"AITalk_AudioEncoder_MulawEncoder_new\0")
            }?,
            AITalk_AudioEncoder_MulawEncoder_delete: *unsafe {
                __library.get(b"AITalk_AudioEncoder_MulawEncoder_delete\0")
            }?,
            AITalk_AudioEncoder_MulawEncoder_addData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_MulawEncoder_addData\0")
            }?,
            AITalk_AudioEncoder_MulawEncoder_endData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_MulawEncoder_endData\0")
            }?,
            AITalk_AudioEncoder_ResamplingEncoderConfig_new: *unsafe {
                __library.get(b"AITalk_AudioEncoder_ResamplingEncoderConfig_new\0")
            }?,
            AITalk_AudioEncoder_ResamplingEncoderConfig_delete: *unsafe {
                __library.get(b"AITalk_AudioEncoder_ResamplingEncoderConfig_delete\0")
            }?,
            AITalk_AudioEncoder_ResamplingEncoder_new: *unsafe {
                __library.get(b"AITalk_AudioEncoder_ResamplingEncoder_new\0")
            }?,
            AITalk_AudioEncoder_ResamplingEncoder_delete: *unsafe {
                __library.get(b"AITalk_AudioEncoder_ResamplingEncoder_delete\0")
            }?,
            AITalk_AudioEncoder_ResamplingEncoder_addData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_ResamplingEncoder_addData\0")
            }?,
            AITalk_AudioEncoder_ResamplingEncoder_endData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_ResamplingEncoder_endData\0")
            }?,
            AITalk_AudioEncoder_WaveEncoder_new: *unsafe {
                __library.get(b"AITalk_AudioEncoder_WaveEncoder_new\0")
            }?,
            AITalk_AudioEncoder_WaveEncoder_delete: *unsafe {
                __library.get(b"AITalk_AudioEncoder_WaveEncoder_delete\0")
            }?,
            AITalk_AudioEncoder_WaveEncoder_addData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_WaveEncoder_addData\0")
            }?,
            AITalk_AudioEncoder_WaveEncoder_endData: *unsafe {
                __library.get(b"AITalk_AudioEncoder_WaveEncoder_endData\0")
            }?,
            __library,
        })
    }
}

use std::ffi::c_int;

pub const AITalk_Core_TtsParameterId_Context: AITalk_Core_TtsParameterIdEnum = 1;
pub const AITalk_Core_TtsParameterId_WaitLast: AITalk_Core_TtsParameterIdEnum = 2;
pub const AITalk_Core_TtsParameterId_WaitTime_BeginText: AITalk_Core_TtsParameterIdEnum = 100;
pub const AITalk_Core_TtsParameterId_WaitTime_EndText: AITalk_Core_TtsParameterIdEnum = 101;
pub const AITalk_Core_TtsParameterId_WaitTime_ShortText: AITalk_Core_TtsParameterIdEnum = 102;
pub const AITalk_Core_TtsParameterId_WaitTime_MiddleText: AITalk_Core_TtsParameterIdEnum = 103;
pub const AITalk_Core_TtsParameterId_WaitTime_LongText: AITalk_Core_TtsParameterIdEnum = 104;
pub const AITalk_Core_TtsParameterId_WaitTime_Eos: AITalk_Core_TtsParameterIdEnum = 105;
pub const AITalk_Core_TtsParameterId_Volume: AITalk_Core_TtsParameterIdEnum = 3;
pub const AITalk_Core_TtsParameterId_VolumeLevel: AITalk_Core_TtsParameterIdEnum = 4;
pub const AITalk_Core_TtsParameterId_Rate: AITalk_Core_TtsParameterIdEnum = 5;
pub const AITalk_Core_TtsParameterId_RateLevel: AITalk_Core_TtsParameterIdEnum = 6;
pub const AITalk_Core_TtsParameterId_Pitch: AITalk_Core_TtsParameterIdEnum = 7;
pub const AITalk_Core_TtsParameterId_PitchLevel: AITalk_Core_TtsParameterIdEnum = 8;
pub const AITalk_Core_TtsParameterId_Emphasis: AITalk_Core_TtsParameterIdEnum = 9;
pub const AITalk_Core_TtsParameterId_EmphasisLevel: AITalk_Core_TtsParameterIdEnum = 10;
pub const AITalk_Core_TtsParameterId_SentenceMaxLength: AITalk_Core_TtsParameterIdEnum = 11;
pub const AITalk_Core_TtsParameterId_CompressorMode: AITalk_Core_TtsParameterIdEnum = 12;
pub const AITalk_Core_TtsParameterId_MasterVolume: AITalk_Core_TtsParameterIdEnum = 13;
pub const AITalk_Core_TtsParameterId_UseRuby: AITalk_Core_TtsParameterIdEnum = 14;
pub const AITalk_Core_TtsParameterId_StyleType: AITalk_Core_TtsParameterIdEnum = 15;
pub const AITalk_Core_TtsParameterId_StyleColor: AITalk_Core_TtsParameterIdEnum = 16;
pub const AITalk_Core_TtsParameterId_EstimatePause: AITalk_Core_TtsParameterIdEnum = 17;
pub const AITalk_Core_TtsParameterId_FixedPauseSettings: AITalk_Core_TtsParameterIdEnum = 18;
pub const AITalk_Core_TtsParameterId_RateMin: AITalk_Core_TtsParameterIdEnum = 19;
pub const AITalk_Core_TtsParameterId_RateMax: AITalk_Core_TtsParameterIdEnum = 20;
pub const AITalk_Core_TtsParameterId_PitchMin: AITalk_Core_TtsParameterIdEnum = 21;
pub const AITalk_Core_TtsParameterId_PitchMax: AITalk_Core_TtsParameterIdEnum = 22;
pub const AITalk_Core_TtsParameterId_EmphasisMax: AITalk_Core_TtsParameterIdEnum = 23;
pub const AITalk_Core_TtsParameterId_UserWordDictionary: AITalk_Core_TtsParameterIdEnum = 300;
pub const AITalk_Core_TtsParameterId_UserPhraseDictionary: AITalk_Core_TtsParameterIdEnum = 301;
pub const AITalk_Core_TtsParameterId_UserSymbolDictionary: AITalk_Core_TtsParameterIdEnum = 302;
pub const AITalk_Core_TtsParameterId_UserKeywordReplacementDictionary:
    AITalk_Core_TtsParameterIdEnum = 303;
pub const AITalk_Core_TtsParameterId_AudioFile: AITalk_Core_TtsParameterIdEnum = 400;

pub type AITalk_Core_TtsParameterIdEnum = c_int;
pub type AITalk_Core_TtsParameterId = i32;

#[repr(C)]
pub struct AITalk_Core_TtsParameter {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub const AITalk_Core_TtsParameter_ContextId_Normal: AITalk_Core_TtsParameter_ContextIdEnum = 1;
pub const AITalk_Core_TtsParameter_ContextId_Address: AITalk_Core_TtsParameter_ContextIdEnum = 2;

pub type AITalk_Core_TtsParameter_ContextIdEnum = c_int;
pub type AITalk_Core_TtsParameter_ContextId = i32;

pub const AITalk_Core_TtsParameter_WaitLastModeId_On: AITalk_Core_TtsParameter_WaitLastModeIdEnum =
    1;
pub const AITalk_Core_TtsParameter_WaitLastModeId_Off: AITalk_Core_TtsParameter_WaitLastModeIdEnum =
    2;

pub type AITalk_Core_TtsParameter_WaitLastModeIdEnum = c_int;
pub type AITalk_Core_TtsParameter_WaitLastModeId = i32;

pub const AITalk_Core_TtsParameter_CompressorModeId_Default:
    AITalk_Core_TtsParameter_CompressorModeIdEnum = 1;
pub const AITalk_Core_TtsParameter_CompressorModeId_Simple:
    AITalk_Core_TtsParameter_CompressorModeIdEnum = 2;

pub type AITalk_Core_TtsParameter_CompressorModeIdEnum = c_int;
pub type AITalk_Core_TtsParameter_CompressorModeId = i32;

pub const AITalk_Core_TtsParameter_UseRubyId_On: AITalk_Core_TtsParameter_UseRubyIdEnum = 1;
pub const AITalk_Core_TtsParameter_UseRubyId_Off: AITalk_Core_TtsParameter_UseRubyIdEnum = 2;

pub type AITalk_Core_TtsParameter_UseRubyIdEnum = c_int;
pub type AITalk_Core_TtsParameter_UseRubyId = i32;

pub const AITalk_Core_TtsParameter_EstimatePause_Estimate:
    AITalk_Core_TtsParameter_EstimatePauseIdEnum = 1;
pub const AITalk_Core_TtsParameter_EstimatePause_Config:
    AITalk_Core_TtsParameter_EstimatePauseIdEnum = 2;

pub type AITalk_Core_TtsParameter_EstimatePauseIdEnum = c_int;
pub type AITalk_Core_TtsParameter_EstimatePauseId = i32;

pub const AITalk_Core_TtsParameter_FixedPauseSettings_FlexibleByRate:
    AITalk_Core_TtsParameter_FixedPauseSettingsIdEnum = 1;
pub const AITalk_Core_TtsParameter_FixedPauseSettings_Fixed:
    AITalk_Core_TtsParameter_FixedPauseSettingsIdEnum = 2;

pub type AITalk_Core_TtsParameter_FixedPauseSettingsIdEnum = c_int;
pub type AITalk_Core_TtsParameter_FixedPauseSettingsId = i32;

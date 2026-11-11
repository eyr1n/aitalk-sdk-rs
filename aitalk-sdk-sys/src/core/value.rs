use std::ffi::c_int;

pub const AITalk_Core_GlobalConfig_Log_Level_Trace: AITalk_Core_GlobalConfig_Log_LevelEnum = 1;
pub const AITalk_Core_GlobalConfig_Log_Level_Verbose: AITalk_Core_GlobalConfig_Log_LevelEnum = 2;
pub const AITalk_Core_GlobalConfig_Log_Level_Info: AITalk_Core_GlobalConfig_Log_LevelEnum = 3;
pub const AITalk_Core_GlobalConfig_Log_Level_Warn: AITalk_Core_GlobalConfig_Log_LevelEnum = 4;
pub const AITalk_Core_GlobalConfig_Log_Level_Error: AITalk_Core_GlobalConfig_Log_LevelEnum = 5;
pub const AITalk_Core_GlobalConfig_Log_Level_Off: AITalk_Core_GlobalConfig_Log_LevelEnum = 6;

pub type AITalk_Core_GlobalConfig_Log_LevelEnum = c_int;
pub type AITalk_Core_GlobalConfig_Log_Level = i32;

pub const AITalk_Core_TtsOutEventId_Begin: AITalk_Core_TtsOutEventIdEnum = 1;
pub const AITalk_Core_TtsOutEventId_BeginSentence: AITalk_Core_TtsOutEventIdEnum = 2;
pub const AITalk_Core_TtsOutEventId_AIKana: AITalk_Core_TtsOutEventIdEnum = 3;
pub const AITalk_Core_TtsOutEventId_Bufreq: AITalk_Core_TtsOutEventIdEnum = 4;
pub const AITalk_Core_TtsOutEventId_Bufdone: AITalk_Core_TtsOutEventIdEnum = 5;
pub const AITalk_Core_TtsOutEventId_Marker: AITalk_Core_TtsOutEventIdEnum = 6;
pub const AITalk_Core_TtsOutEventId_EndSentence: AITalk_Core_TtsOutEventIdEnum = 7;
pub const AITalk_Core_TtsOutEventId_End: AITalk_Core_TtsOutEventIdEnum = 8;
pub const AITalk_Core_TtsOutEventId_SwitchParameterBegin: AITalk_Core_TtsOutEventIdEnum = 9;
pub const AITalk_Core_TtsOutEventId_SwitchParameterEnd: AITalk_Core_TtsOutEventIdEnum = 10;

pub type AITalk_Core_TtsOutEventIdEnum = c_int;
pub type AITalk_Core_TtsOutEventId = i32;

pub const AITalk_Core_MarkerModeId_Disable: AITalk_Core_MarkerModeIdEnum = 1;
pub const AITalk_Core_MarkerModeId_OutBuffer: AITalk_Core_MarkerModeIdEnum = 2;
pub const AITalk_Core_MarkerModeId_CallbackAndBuffer: AITalk_Core_MarkerModeIdEnum = 3;

pub type AITalk_Core_MarkerModeIdEnum = c_int;
pub type AITalk_Core_MarkerModeId = i32;

pub const AITalk_Core_MarkerTypeId_Bookmark: AITalk_Core_MarkerTypeIdEnum = 1;
pub const AITalk_Core_MarkerTypeId_Lipsync: AITalk_Core_MarkerTypeIdEnum = 2;
pub const AITalk_Core_MarkerTypeId_Accent: AITalk_Core_MarkerTypeIdEnum = 3;
pub const AITalk_Core_MarkerTypeId_Phrase: AITalk_Core_MarkerTypeIdEnum = 4;

pub type AITalk_Core_MarkerTypeIdEnum = c_int;
pub type AITalk_Core_MarkerTypeId = i32;

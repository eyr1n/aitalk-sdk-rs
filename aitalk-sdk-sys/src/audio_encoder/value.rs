use std::ffi::c_int;

pub const AITalk_AudioEncoder_SoundFormatType_Lpcm: AITalk_AudioEncoder_SoundFormatTypeEnum = 1;
pub const AITalk_AudioEncoder_SoundFormatType_Mulaw: AITalk_AudioEncoder_SoundFormatTypeEnum = 7;

pub type AITalk_AudioEncoder_SoundFormatTypeEnum = c_int;
pub type AITalk_AudioEncoder_SoundFormatType = u16;

pub const AITalk_AudioEncoder_EncoderEventId_Bufreq: AITalk_AudioEncoder_EncoderEventIdEnum = 1;
pub const AITalk_AudioEncoder_EncoderEventId_Bufdone: AITalk_AudioEncoder_EncoderEventIdEnum = 2;

pub type AITalk_AudioEncoder_EncoderEventIdEnum = c_int;
pub type AITalk_AudioEncoder_EncoderEventId = i32;

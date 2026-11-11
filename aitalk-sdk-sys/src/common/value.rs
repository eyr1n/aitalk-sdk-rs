use std::ffi::c_int;

pub const AITalk_TextEncodingsId_Utf8: AITalk_TextEncodingsIdEnum = 1;
pub const AITalk_TextEncodingsId_Cp932: AITalk_TextEncodingsIdEnum = 2;
pub const AITalk_TextEncodingsId_Default: AITalk_TextEncodingsIdEnum = 3;

pub type AITalk_TextEncodingsIdEnum = c_int;
pub type AITalk_TextEncodingsId = i32;

pub const AITalk_TypeOfInputId_Plain: AITalk_TypeOfInputIdEnum = 1;
pub const AITalk_TypeOfInputId_AIKana: AITalk_TypeOfInputIdEnum = 2;
pub const AITalk_TypeOfInputId_Jeita: AITalk_TypeOfInputIdEnum = 3;

pub type AITalk_TypeOfInputIdEnum = c_int;
pub type AITalk_TypeOfInputId = i32;

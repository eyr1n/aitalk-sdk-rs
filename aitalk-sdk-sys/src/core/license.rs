use std::ffi::c_int;

pub const AITalk_Core_EngineLicenseId_Path: AITalk_Core_EngineLicenseIdEnum = 1;
pub const AITalk_Core_EngineLicenseId_Code: AITalk_Core_EngineLicenseIdEnum = 2;

pub type AITalk_Core_EngineLicenseIdEnum = c_int;
pub type AITalk_Core_EngineLicenseId = i32;

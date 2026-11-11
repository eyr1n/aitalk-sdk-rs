use std::ffi::{c_char, c_int, c_void};

use crate::AITalk_Core_GlobalConfig_Log_Level;

pub const AITalkGlobalConfigId_Log: AITalkGlobalConfigIdEnum = 1;

pub type AITalkGlobalConfigIdEnum = c_int;
pub type AITalkGlobalConfigId = i32;

pub const AITalk_Core_GlobalConfig_Log_Type_StdOut: AITalk_Core_GlobalConfig_Log_TypeEnum = 1;
pub const AITalk_Core_GlobalConfig_Log_Type_StdErr: AITalk_Core_GlobalConfig_Log_TypeEnum = 2;
pub const AITalk_Core_GlobalConfig_Log_Type_Callback: AITalk_Core_GlobalConfig_Log_TypeEnum = 3;

pub type AITalk_Core_GlobalConfig_Log_TypeEnum = c_int;
pub type AITalk_Core_GlobalConfig_Log_Type = i32;
pub type AITalk_Core_GlobalConfig_Log_CallbackFunc = Option<
    unsafe extern "C" fn(
        arg: *mut c_void,
        level: i32,
        rtype: *const c_char,
        rtype_size: usize,
        time: *const c_char,
        time_size: usize,
        message: *const c_char,
        message_size: usize,
    ),
>;

#[repr(C)]
pub struct AITalk_Core_GlobalConfig_Log {
    pub log_type: AITalk_Core_GlobalConfig_Log_Type,
    pub log_level: AITalk_Core_GlobalConfig_Log_Level,
    pub log_callback: AITalk_Core_GlobalConfig_Log_CallbackFunc,
    pub user_data: *mut c_void,
}

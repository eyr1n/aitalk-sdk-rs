use std::ffi::c_int;

pub const AITalk_Core_VoiceDictionaryMetaContainerId_Name:
    AITalk_Core_VoiceDictionaryMetaContainerIdEnum = 1;
pub const AITalk_Core_VoiceDictionaryMetaContainerId_Gender:
    AITalk_Core_VoiceDictionaryMetaContainerIdEnum = 2;
pub const AITalk_Core_VoiceDictionaryMetaContainerId_Rate:
    AITalk_Core_VoiceDictionaryMetaContainerIdEnum = 3;
pub const AITalk_Core_VoiceDictionaryMetaContainerId_StyleColorLabels:
    AITalk_Core_VoiceDictionaryMetaContainerIdEnum = 4;

pub type AITalk_Core_VoiceDictionaryMetaContainerIdEnum = c_int;
pub type AITalk_Core_VoiceDictionaryMetaContainerId = i32;

#[repr(C)]
pub struct AITalk_Core_VoiceDictionaryMetaContainer {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

use std::ffi::c_int;

pub const AITalk_Core_LanguageDictionaryMetaContainerId_Version:
    AITalk_Core_LanguageDictionaryMetaContainerIdEnum = 1;

pub type AITalk_Core_LanguageDictionaryMetaContainerIdEnum = c_int;
pub type AITalk_Core_LanguageDictionaryMetaContainerId = i32;

#[repr(C)]
pub struct AITalk_Core_LanguageDictionaryMetaContainer {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

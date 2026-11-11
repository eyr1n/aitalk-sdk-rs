use std::ffi::{NulError, OsString};
use std::io;
use std::num::TryFromIntError;

use aitalk_sdk_sys::{AITalkReturnCode, AITalkReturnCode_Ok};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Nul(#[from] NulError),
    #[error(transparent)]
    TryFromInt(#[from] TryFromIntError),
    #[error("string was not valid ANSI: {0:?}")]
    NotAnsi(OsString),
    #[error("SDK returned error code: {0}")]
    Sdk(AITalkReturnCode),
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn to_result(code: AITalkReturnCode) -> Result<()> {
    if code == AITalkReturnCode_Ok {
        Ok(())
    } else {
        Err(Error::Sdk(code))
    }
}

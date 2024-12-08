use crate::device_syslog::LoggerCommand;
use crossbeam_channel::SendError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceSysLogError {
    #[error("Couldn't send a message to the channel, maybe it's closed?, error: {0}")]
    SendError(#[from] SendError<LoggerCommand>),
}

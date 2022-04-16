use crate::LogLevel;

use std::io;

/// Trait for implementing a logging utility.
///
/// This trait has only one method, that is [`log`]. This method is called whenever
/// some operation, that may be important, happens.
///
/// Note that [`Logger`] trait does not specify how given message is logged nor
/// where it should be logged to.
///
/// [`log`]: Logger::log
pub trait Logger: Send + Sync {
    /// Method for logging a message.
    ///
    /// # Parameters
    ///
    ///  -  `log_level`: [`LogLevel`] of given message
    ///  -  `message`: message text to be logged
    ///
    /// # Return value
    ///
    /// [`std::io::Result`]`<()>` indicating, whether the message was successfully logged
    fn log(&mut self, log_level: LogLevel, message: &str) -> io::Result<()>;
}
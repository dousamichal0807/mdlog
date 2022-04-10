pub mod composite;
pub mod text;

pub use self::composite::CompositeLogger;
pub use self::text::TextLogger;

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;

/// Trait for implementing a logging utility.
///
/// This trait has only one method, that is [`log`]. This method is called whenever
/// some operation, that may be important, happens.
///
/// Note that [`Logger`] trait does not specify how given message is logged nor
/// where it should be logged to.
pub trait Logger {
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

/// Represents how important a log message is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogLevel {
    /// [`LogLevel`] for debugging purposes.
    Debug,

    /// [`LogLevel`] for informational messages.
    Info,

    /// [`LogLevel`] for messages that warns the user of application about potential risks.
    Warning,

    /// [`LogLevel`] for messages that informs the user that something went wrong and the error is
    /// cannot be corrected, but the application can continue running.
    Error,

    /// [`LogLevel`] for messages that informs the user that something went wrong and the error is
    /// unrecoverable and the application cannot continue.
    Fatal,
}

impl LogLevel {
    /// Describes the [`LogLevel`] as a number ([`u8`]).
    ///
    /// # Return value
    ///
    /// [`LogLevel`] represented as an [`u8`]
    pub fn numeric_level(&self) -> u8 {
        match self {
            Self::Debug => 0,
            Self::Info => 1,
            Self::Warning => 2,
            Self::Error => 3,
            Self::Fatal => 4,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}",
                 match self {
                     Self::Debug => "DEBUG",
                     Self::Info => "INFO",
                     Self::Warning => "WARN",
                     Self::Error => "ERROR",
                     Self::Fatal => "FATAL",
                 }
        )
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.cmp(other))
    }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.numeric_level().cmp(&other.numeric_level())
    }
}
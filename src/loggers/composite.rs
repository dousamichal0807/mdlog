use std::io;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::Logger;
use crate::LogLevel;

type LoggerVec = Vec<Box<dyn Logger + Send + Sync>>;

/// Represents a [`Logger`] that consists of many [`Logger`]s.
///
/// [`CompositeLogger`] is used through its [`Deref`] and [`DerefMut`]
/// implementations which yield a borrow to the inner [`Vec`] of [`Logger`]s.
pub struct CompositeLogger (LoggerVec);

impl CompositeLogger {
    /// Creates a new [`CompositeLogger`] instance.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new [`CompositeLogger`] with given capacity of the underlying
    /// [`Vec`]. See [`Vec::with_capacity`] method for more information.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl Logger for CompositeLogger {
    fn log(&mut self, log_level: LogLevel, message: &str) -> io::Result<()> {
        for logger in &mut self.0 {
            logger.log(log_level, message)?;
        }
        Result::Ok(())
    }
}

impl Deref for CompositeLogger {
    type Target = LoggerVec;

    fn deref(&self) -> &LoggerVec {
        &self.0
    }
}

impl DerefMut for CompositeLogger {
    fn deref_mut(&mut self) -> &mut LoggerVec {
        &mut self.0
    }
}
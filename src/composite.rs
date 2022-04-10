use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::io;

use crate::Logger;
use crate::LogLevel;

type LoggerVec = Vec<Box<dyn Logger + Send + Sync>>;

/// Represents a [`Logger`] that consists of many [`Logger`]s.
///
/// [`CompositeLogger`] is used through its [`Borrow`] and [`BorrowMut`]
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

impl Borrow<LoggerVec> for CompositeLogger {
    fn borrow(&self) -> &LoggerVec {
        &self.0
    }
}

impl BorrowMut<LoggerVec> for CompositeLogger {
    fn borrow_mut(&mut self) -> &mut LoggerVec {
        &mut self.0
    }
}
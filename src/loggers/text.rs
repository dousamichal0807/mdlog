use std::io;
use std::io::Write;
use std::thread;

use chrono::Local;

use crate::Logger;
use crate::LogLevel;

/// [`Logger`] implementation that writes log messages formatted as UTF-8 to given
/// output stream. The output stream can be anything, from [`Stdout`] to [`File`] or
/// [`TcpStream`].
///
/// [`File`]: std::fs::File
/// [`Stdout`]: std::io::Stdout
/// [`TcpStream`]: std::net::TcpStream
///
/// # Usage
///
/// ```rust
/// use mdlog::Logger;
/// use mdlog::LogLevel;
/// use mdlog::loggers::TextLogger;
///
/// use std::io::stdout;
///
/// fn main() {
///     let mut logger = TextLogger::new(LogLevel::Warning, stdout());
///     // These messages will not get printed, because their log level is too low:
///     logger.log(LogLevel::Debug, "This is a debug message");
///     logger.log(LogLevel::Info, "This is an informational message");
///     // These will get printed, because their log level is high enough:
///     logger.log(LogLevel::Warning, "Something suspicious happened");
///     logger.log(LogLevel::Error, "Oh, no! An error occurred!");
///     logger.log(LogLevel::Fatal, "NO! FATAL ERROR! Application is shut down!");
/// }
/// ```
pub struct TextLogger<W>
    where W: Write {
    min_log_level: LogLevel,
    writer: W,
}

impl<W> TextLogger<W>
    where W: Write {
    /// Creates a new [`TextLogger`] instance with given name, minimum log level and
    /// output stream.
    ///
    /// # Parameters
    ///
    /// - `name`: name of the logger
    /// - `min_log_level`: minimum log level
    /// - `writer`: an output stream the log is written into
    pub fn new(min_log_level: LogLevel, writer: W) -> Self {
        Self {
            min_log_level,
            writer,
        }
    }

    /// Returns the minimum log level of the [`Logger`] instance.
    pub fn min_log_level(&self) -> LogLevel {
        self.min_log_level
    }
}

impl<W> Logger for TextLogger<W>
    where W: Write {
    fn log(&mut self, log_level: LogLevel, message: &str) -> io::Result<()> {
        // Do not do anything if log level is too low:
        if log_level < self.min_log_level { return Result::Ok(()) }
        // Write to the specified stream:
        writeln!(self.writer, "[{} {}]@{}: {}",
                 thread::current().name().unwrap_or(""),
                 log_level, Local::now(), message)
    }
}
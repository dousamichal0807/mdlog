/*
 * Copyright (c) 2022 Michal Douša. All rights reserved.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
///
/// [`File`]: std::fs::File
/// [`Stdout`]: std::io::Stdout
/// [`TcpStream`]: std::net::TcpStream
pub struct TextLogger<W>
where
    W: Send + Sync + Write
{
    min_log_level: LogLevel,
    writer: W,
}

impl<W> TextLogger<W>
where
    W: Write + Send + Sync
{
    /// Creates a new [`TextLogger`] instance with given name, minimum log level and
    /// output stream.
    ///
    /// # Parameters
    ///
    /// - `name`: name of the logger
    /// - `min_log_level`: minimum log level
    /// - `writer`: an output stream the log is written into
    pub fn new(min_log_level: LogLevel, writer: W) -> Self {
        Self { min_log_level, writer }
    }

    /// Returns the minimum log level of the [`Logger`] instance.
    pub fn min_log_level(&self) -> LogLevel {
        self.min_log_level
    }
}

impl<W> Logger for TextLogger<W>
where
    W: Write + Send + Sync
{
    fn log(&mut self, log_level: LogLevel, message: &str) -> io::Result<()> {
        // Do not do anything if log level is too low:
        if log_level < self.min_log_level { return Result::Ok(()) }
        // Construct what to show as category:
        let category = thread::current().name()
            .map(|name| format!(" {}", name))
            .unwrap_or_default();
        // Write to the specified stream:
        writeln!(self.writer, "{} [{} {}]: {}", Local::now(), log_level, category, message)
    }
}
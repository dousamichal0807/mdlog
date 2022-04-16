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
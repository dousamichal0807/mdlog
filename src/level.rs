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

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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
        write!(f, "{}", match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warning => "WARN",
            Self::Error => "ERROR",
            Self::Fatal => "FATAL",
        })
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
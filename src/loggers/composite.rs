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
    ///
    /// # Parameters
    ///
    ///  -  `capacity`: capacity of the underlying [`Vec`]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn add<L>(&mut self, logger: L)
    where L: Logger {
        self.0.push(logger)
    }

    /// Merges `self` with another instance of [`CompositeLogger`].
    ///
    /// # Parameters
    ///
    ///  -  `other`: the other instance of [`CompositeLogger`] to merge with
    pub fn append(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
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
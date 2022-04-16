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

//! A simple crate for logging.
//!
//! # The [`Logger`] trait
//!
//! All loggers implement [`Logger`] trait. See [`Logger`] trait for more
//! information.
//!
//! # Featured logger types
//!
//!  -  [`CompositeLogger`](crate::loggers::CompositeLogger)
//!  -  [`TextLogger`](crate::loggers::TextLogger)
#![forbid(unsafe_code)]
#![forbid(unused_crate_dependencies)]
#![forbid(unused_extern_crates)]

pub mod auto;
pub mod level;
pub mod loggers;

pub use crate::auto::Logger;
pub use crate::level::LogLevel;
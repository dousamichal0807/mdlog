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
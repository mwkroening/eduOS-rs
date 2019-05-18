// Copyright (c) 2017-2018 Stefan Lankes, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Basic error handling

use core::{result, fmt};

pub type Result<T> = result::Result<T, Error>;

/// Possible errors of eduOS-rs
#[derive(Debug,Clone)]
pub enum Error {
	/// Usage of a invalid priority
	BadPriority
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::BadPriority => write!(f, "Invalid priority number")
		}
	}
}

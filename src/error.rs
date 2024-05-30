use std::{
	fmt::{Debug, Formatter, Result},
	io,
};

pub enum Error {
	FileSystem,
	Malformed,
	Unsupported,
}

impl Debug for Error {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
		match self {
			Error::Malformed => write!(formatter, "malformed file"),
			Error::Unsupported => write!(formatter, "format not supported"),
			Error::FileSystem => write!(formatter, "could not read file"),
		}
	}
}

impl From<io::Error> for Error {
	fn from(_: io::Error) -> Self {
		Error::FileSystem
	}
}

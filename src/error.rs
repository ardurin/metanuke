use std::{
	fmt::{Debug, Formatter, Result},
	io,
};
use zip::result::ZipError;

pub enum Error {
	Encrypted,
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
			Error::Encrypted => write!(formatter, "encrypted PDFs are not supported"),
		}
	}
}

impl From<io::Error> for Error {
	fn from(_: io::Error) -> Self {
		Error::FileSystem
	}
}

impl From<lopdf::Error> for Error {
	fn from(_: lopdf::Error) -> Self {
		Error::Malformed
	}
}

impl From<ZipError> for Error {
	fn from(error: ZipError) -> Self {
		match error {
			ZipError::Io(_) | ZipError::FileNotFound => Error::FileSystem,
			_ => Error::Malformed,
		}
	}
}

use crate::{error::Error, metadata::*};
use std::{
	ffi::OsStr,
	fs::File,
	io::{BufReader, BufWriter},
	path::Path,
};

type Function = fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<(), Error>;

pub fn identify<P: AsRef<Path>>(path: P) -> Result<Function, Error> {
	match path.as_ref().extension().and_then(OsStr::to_str) {
		Some("jpg") => Ok(jpeg::delete_metadata),
		Some("jpeg") => Ok(jpeg::delete_metadata),
		Some("png") => Ok(png::delete_metadata),
		_ => Err(Error::Unsupported),
	}
}

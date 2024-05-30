use crate::error::Error;
use std::io::{Seek, SeekFrom};

pub fn skip<S: Seek>(source: &mut S, size: u64) -> Result<(), Error> {
	let position = source.seek(SeekFrom::Current(0))?;
	if source.seek(SeekFrom::Current(size as i64))? < position + size {
		return Err(Error::Malformed);
	}
	Ok(())
}

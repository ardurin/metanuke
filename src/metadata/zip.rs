use crate::Error;
use std::io::{Read, Seek, Write};
use zip::{DateTime, ZipArchive, ZipWriter};

pub fn delete_metadata<R: Read + Seek, W: Write + Seek>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	let mut source = ZipArchive::new(source)?;
	let mut destination = ZipWriter::new(destination);
	for i in 0..source.len() {
		let entry = source.by_index(i)?;
		destination.raw_copy_file_touch(entry, DateTime::default(), None)?;
	}
	destination.finish()?;

	Ok(())
}

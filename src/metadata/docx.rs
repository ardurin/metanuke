use crate::{metadata::office, Error};
use std::io::{Read, Seek, Write};

pub fn delete_metadata<R: Read + Seek, W: Write + Seek>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	office::delete_metadata(source, destination, "word/")
}

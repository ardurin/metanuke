use crate::{
	error::Error,
	util::{read, skip},
};
use std::io::{copy, Read, Seek, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	if copy(&mut source.take(4), destination)? < 4 {
		return Err(Error::Malformed);
	}
	let mut keep = true;
	let mut pending_metadata = true;
	while pending_metadata {
		let mut header: [u8; 4] = [0; 4];
		if !read(source, &mut header)? {
			break;
		}
		pending_metadata = header[0] < 128;
		let mut size = header;
		size[0] = 0;
		let size = u32::from_be_bytes(size);
		if keep {
			keep = false;
			if header[0] & 0b01111111 == 0 {
				destination.write_all(&[0b10000000])?;
				destination.write_all(&header[1..4])?;
				copy(&mut source.take(size.into()), destination)?;
			} else {
				return Err(Error::Malformed);
			}
		} else {
			skip(source, size.into())?;
		}
	}
	copy(source, destination)?;

	Ok(())
}

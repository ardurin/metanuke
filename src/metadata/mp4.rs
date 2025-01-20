use crate::{
	error::Error,
	util::{read, skip},
};
use std::io::{copy, Read, Seek, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	while process_box(source, destination)? > 0 {}
	Ok(())
}

fn process_box<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<u64, Error> {
	let mut size: [u8; 4] = [0; 4];
	if !read(source, &mut size)? {
		return Ok(0);
	}
	let mut size = u32::from_be_bytes(size) as u64;
	let mut header_size = 8;

	let mut name: [u8; 4] = [0; 4];
	if !read(source, &mut name)? {
		return Err(Error::Malformed);
	}

	if size == 1 {
		let mut data: [u8; 8] = [0; 8];
		if !read(source, &mut data)? {
			return Err(Error::Malformed);
		}
		size = u64::from_be_bytes(data);
		header_size = 16;
	}

	match &name {
		b"free" | b"meco" | b"meta" | b"skip" => {
			skip(source, size - header_size)?;
		}
		b"moof" | b"moov" | b"traf" | b"trak" => {
			let mut count = 0;
			let mut data = Vec::new();
			while count < size - header_size {
				let child_size = process_box(source, &mut data)?;
				if child_size == 0 {
					return Err(Error::Malformed);
				}
				count += child_size
			}
			if data.len() > u32::MAX as usize {
				destination.write(&[0, 0, 0, 1])?;
				destination.write(&name)?;
				destination.write(&(data.len() as u64 + 16).to_be_bytes())?;
			} else {
				destination.write(&(data.len() as u32 + 8).to_be_bytes())?;
				destination.write(&name)?;
			}
			destination.write(&data)?;
		}
		_ => {
			if header_size == 16 {
				destination.write(&[0, 0, 0, 1])?;
				destination.write(&name)?;
				destination.write(&size.to_be_bytes())?;
			} else {
				destination.write(&(size as u32).to_be_bytes())?;
				destination.write(&name)?;
			}
			let size = size - header_size;
			if copy(&mut source.take(size), destination)? < size {
				return Err(Error::Malformed);
			}
		}
	};

	Ok(size)
}

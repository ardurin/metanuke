use crate::error::Error;
use crate::util::{read, read_u16, read_u8, skip};
use std::io::{self, Read, Seek, SeekFrom, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	loop {
		let mut marker: [u8; 2] = [0; 2];
		if !read(source, &mut marker)? {
			break;
		}

		match marker[1] {
			0xC0..=0xCF => {
				copy_header(source, destination, &marker)?;
			}
			0xD0..=0xD7 => {
				destination.write_all(&marker)?;
				copy_data(source, destination)?;
			}
			0xD8..=0xD9 => {
				destination.write_all(&marker)?;
			}
			0xDA => {
				copy_header(source, destination, &marker)?;
				copy_data(source, destination)?;
			}
			0xDB..=0xDF => {
				copy_header(source, destination, &marker)?;
			}
			/*
			0xE0 => {
				let size = read_u16(source)?;
				if size >= 16 {
					let mut identifier: [u8; 5] = [0; 5];
					source.read(&mut identifier)?;
					match &identifier {
						b"JFIF\0" => {
							destination.write(&[0xFF, 0xE0, 0, 16, b'J', b'F', b'I', b'F', 0])?;
							if io::copy(&mut source.take(7), destination)? < 7 {
								return Err(Error::Malformed);
							}
							destination.write(&[0, 0])?;
							skip(source, size as u64 - 14)?;
						}
						_ => {
							skip(source, size as u64 - 7)?;
						}
					}
				} else {
					skip(source, size as u64 - 2)?;
				}
			}
			*/
			_ => {
				let size = read_u16(source)?;
				skip(source, size as u64 - 2)?;
			}
		}
	}
	Ok(())
}

fn copy_header<R: Read, W: Write>(
	source: &mut R,
	destination: &mut W,
	marker: &[u8; 2],
) -> Result<(), Error> {
	destination.write_all(marker)?;
	let size = read_u16(source)?;
	destination.write_all(&size.to_be_bytes())?;
	if size > 2 {
		let size = size as u64 - 2;
		if io::copy(&mut source.take(size), destination)? < size {
			return Err(Error::Malformed);
		}
	}
	Ok(())
}

fn copy_data<R: Read + Seek, W: Write>(source: &mut R, destination: &mut W) -> Result<(), Error> {
	loop {
		let value = read_u8(source)?;
		if value == 0xFF {
			let next = read_u8(source)?;
			if next == 0 {
				destination.write_all(&[value, next])?;
			} else {
				source.seek(SeekFrom::Current(-2))?;
				break;
			}
		} else {
			destination.write_all(&[value])?;
		}
	}
	Ok(())
}

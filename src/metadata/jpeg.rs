use crate::error::Error;
use crate::util::{read_u16, read_u8, skip};
use std::io::{self, Read, Seek, SeekFrom, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	loop {
		let mut marker: [u8; 2] = [0; 2];
		let count = source.read(&mut marker)?;
		if count == 0 {
			break;
		}
		if count < 2 {
			return Err(Error::Malformed);
		};

		match marker[1] {
			0xC0..=0xCF => {
				copy(source, destination, &marker)?;
			}
			0xD0..=0xD9 => {
				destination.write(&marker)?;
			}
			0xDA => {
				copy(source, destination, &marker)?;
				loop {
					let value = read_u8(source)?;
					if value == 0xFF {
						let next = read_u8(source)?;
						if next == 0 {
							destination.write(&[value, next])?;
						} else {
							source.seek(SeekFrom::Current(-2))?;
							break;
						}
					} else {
						destination.write(&[value])?;
					}
				}
			}
			0xDB..=0xDF => {
				copy(source, destination, &marker)?;
			}
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
			_ => {
				let size = read_u16(source)?;
				skip(source, size as u64 - 2)?;
			}
		}
	}
	Ok(())
}

fn copy<T: Read, U: Write>(
	source: &mut T,
	destination: &mut U,
	marker: &[u8; 2],
) -> Result<(), Error> {
	destination.write(marker)?;
	let size = read_u16(source)?;
	destination.write(&size.to_be_bytes())?;
	if size > 2 {
		let size = size as u64 - 2;
		if io::copy(&mut source.take(size), destination)? < size {
			return Err(Error::Malformed);
		}
	}
	Ok(())
}

use crate::{
	error::Error,
	util::{read, read_u32},
};
use std::io::{copy, Read, Seek, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	source.seek_relative(8)?;
	let mut code: [u8; 4] = [0; 4];
	source.read_exact(&mut code)?;
	if &code != b"WEBP" {
		return Err(Error::Unsupported);
	}
	let mut data = Vec::new();
	while chunk(source, &mut data)? {}
	let size = data.len() as u32 + 4;
	destination.write_all(b"RIFF")?;
	destination.write_all(&size.to_le_bytes())?;
	destination.write_all(b"WEBP")?;
	destination.write_all(&data)?;
	Ok(())
}

fn chunk<R: Read + Seek, W: Write>(source: &mut R, destination: &mut W) -> Result<bool, Error> {
	let mut code: [u8; 4] = [0; 4];
	if !read(source, &mut code)? {
		return Ok(false);
	}
	let size = read_u32(source)?;
	let total_size = if size % 2 > 0 { size + 1 } else { size };
	match &code {
		b"ALPH" | b"ANIM" | b"ANMF" | b"ICCP" | b"VP8 " | b"VP8L" | b"VP8X" => {
			destination.write_all(&code)?;
			destination.write_all(&size.to_le_bytes())?;
			if copy(&mut source.take(total_size as u64), destination)? < total_size as u64 {
				return Err(Error::Malformed);
			}
		}
		_ => {
			source.seek_relative(total_size as i64)?;
		}
	}
	Ok(true)
}

use crate::{error::Error, util::skip};
use std::io::{copy, Read, Seek, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	if copy(&mut source.take(8), destination)? < 8 {
		return Err(Error::Malformed);
	}
	loop {
		let mut size: [u8; 4] = [0; 4];
		let count = source.read(&mut size)?;
		if count == 0 {
			break;
		}
		if count < 4 {
			return Err(Error::Malformed);
		}
		let size = u32::from_be_bytes(size);

		let mut chunk: [u8; 4] = [0; 4];
		if source.read(&mut chunk)? < 4 {
			return Err(Error::Malformed);
		}

		match &chunk {
			b"IDAT" | b"IEND" | b"IHDR" | b"PLTE" | b"acTL" | b"bKGD" | b"cHRM" | b"cICP"
			| b"fRAc" | b"fcTL" | b"fdAT" | b"gAMA" | b"gIFg" | b"iCCP" | b"sBIT" | b"sRGB"
			| b"sTER" | b"tRNS" => {
				destination.write(&size.to_be_bytes())?;
				destination.write(&chunk)?;
				let size = size as u64 + 4;
				if copy(&mut source.take(size), destination)? < size {
					return Err(Error::Malformed);
				}
			}
			_ => {
				skip(source, size as u64 + 4)?;
			}
		}
	}

	Ok(())
}

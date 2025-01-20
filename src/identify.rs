use crate::{error::Error, metadata::*};
use std::{
	ffi::OsStr,
	fs::File,
	io::{BufReader, BufWriter, Read},
	path::Path,
};

type Function = fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<(), Error>;

pub fn identify<P: AsRef<Path>>(path: P) -> Result<Function, Error> {
	let mut source = File::open(&path)?;
	let mut signature = [0; 8];
	if source.read(&mut signature)? < 8 {
		return Err(Error::Unsupported);
	}
	match signature[0] {
		b'%' => {
			if &signature[1..5] == b"PDF-" {
				return Ok(pdf::delete_metadata);
			}
		}
		0x89 => {
			if signature[1..8] == [0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
				return Ok(png::delete_metadata);
			}
		}
		0xFF => {
			if signature[1..3] == [0xD8, 0xFF] {
				return Ok(jpeg::delete_metadata);
			}
		}
		0x50 => {
			if signature[1..4] == [0x4B, 0x03, 0x04] {
				match path.as_ref().extension().and_then(OsStr::to_str) {
					Some("docx") => return Ok(docx::delete_metadata),
					Some("xlsx") => return Ok(xlsx::delete_metadata),
					_ => {}
				}
			}
		}
		_ => {
			if &signature[4..8] == b"ftyp" {
				return Ok(mp4::delete_metadata);
			}
		}
	}
	Err(Error::Unsupported)
}

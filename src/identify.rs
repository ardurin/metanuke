use crate::{error::Error, metadata::*};
use std::{
	ffi::OsStr,
	fs::File,
	io::{BufReader, BufWriter, ErrorKind, Read},
	path::Path,
};

type Function = fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<(), Error>;

pub fn identify<P: AsRef<Path>>(path: P) -> Result<Function, Error> {
	let mut source = File::open(&path)?;
	let mut signature = [0; 8];
	if let Err(error) = source.read_exact(&mut signature) {
		if error.kind() == ErrorKind::UnexpectedEof {
			return Err(Error::Unsupported);
		}
		return Err(Error::FileSystem);
	};
	match signature[0] {
		b'%' => {
			if &signature[1..5] == b"PDF-" {
				return Ok(pdf::delete_metadata);
			}
		}
		b'I' => {
			if &signature[1..3] == b"D3" {
				return Ok(mp3::delete_metadata);
			}
		}
		b'R' => {
			if &signature[1..4] == b"IFF" {
				return Ok(webp::delete_metadata);
			}
		}
		b'f' => {
			if &signature[1..4] == b"LaC" {
				return Ok(flac::delete_metadata);
			}
		}
		0x89 => {
			if signature[1..8] == [0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
				return Ok(png::delete_metadata);
			}
		}
		0xFF => match signature[1] {
			0xD8 => {
				if signature[2] == 0xFF {
					return Ok(jpeg::delete_metadata);
				}
			}
			0xF2 | 0xF3 | 0xFA | 0xFB => return Ok(mp3::delete_metadata),
			_ => {}
		},
		0x50 => {
			if signature[1..4] == [0x4B, 0x03, 0x04] {
				match path.as_ref().extension().and_then(OsStr::to_str) {
					Some("docx") => return Ok(docx::delete_metadata),
					Some("xlsx") => return Ok(xlsx::delete_metadata),
					_ => return Ok(zip::delete_metadata),
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

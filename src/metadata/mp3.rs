use crate::{
	error::Error,
	util::{read, skip},
};
use std::io::{copy, Read, Seek, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	loop {
		let mut data = [0; 4];
		if !read(source, &mut data)? {
			break;
		}
		if data[0] == 0xFF {
			let size = parse_header(&data)?;
			destination.write(&data)?;
			copy(&mut source.take(size as u64 - 4), destination)?;
		} else if &data[0..3] == b"ID3" {
			skip(source, 2)?;
			let mut data = [0; 4];
			if !read(source, &mut data)? {
				return Err(Error::Malformed);
			}
			let size = size(&data);
			skip(source, size as u64)?;
		} else if &data[0..3] == b"TAG" {
			break;
		} else {
			return Err(Error::Malformed);
		}
	}

	Ok(())
}

enum Version {
	I,
	II,
}

enum Layer {
	I,
	II,
	III,
}

fn parse_header(header: &[u8; 4]) -> Result<usize, Error> {
	let version = match header[1] & 0b00011000 {
		0b00011000 => Version::I,
		0b00010000 => Version::II,
		_ => return Err(Error::Malformed),
	};

	let layer = match header[1] & 0b00000110 {
		0b00000110 => Layer::I,
		0b00000100 => Layer::II,
		0b00000010 => Layer::III,
		_ => return Err(Error::Malformed),
	};

	let value = header[2] & 0b11110000;
	let bitrate: usize = match (&version, &layer) {
		(Version::I, Layer::I) => match value {
			0b00010000 => 32,
			0b00100000 => 64,
			0b00110000 => 96,
			0b01000000 => 128,
			0b01010000 => 160,
			0b01100000 => 192,
			0b01110000 => 224,
			0b10000000 => 256,
			0b10010000 => 288,
			0b10100000 => 320,
			0b10110000 => 352,
			0b11000000 => 384,
			0b11010000 => 416,
			0b11100000 => 448,
			_ => return Err(Error::Malformed),
		},
		(Version::I, Layer::II) => match value {
			0b00010000 => 32,
			0b00100000 => 48,
			0b00110000 => 56,
			0b01000000 => 64,
			0b01010000 => 80,
			0b01100000 => 96,
			0b01110000 => 112,
			0b10000000 => 128,
			0b10010000 => 160,
			0b10100000 => 192,
			0b10110000 => 224,
			0b11000000 => 256,
			0b11010000 => 320,
			0b11100000 => 384,
			_ => return Err(Error::Malformed),
		},
		(Version::I, Layer::III) => match value {
			0b00010000 => 32,
			0b00100000 => 40,
			0b00110000 => 48,
			0b01000000 => 56,
			0b01010000 => 64,
			0b01100000 => 80,
			0b01110000 => 96,
			0b10000000 => 112,
			0b10010000 => 128,
			0b10100000 => 160,
			0b10110000 => 192,
			0b11000000 => 224,
			0b11010000 => 256,
			0b11100000 => 320,
			_ => return Err(Error::Malformed),
		},
		(Version::II, Layer::I) => match value {
			0b00010000 => 32,
			0b00100000 => 48,
			0b00110000 => 56,
			0b01000000 => 64,
			0b01010000 => 80,
			0b01100000 => 96,
			0b01110000 => 112,
			0b10000000 => 128,
			0b10010000 => 144,
			0b10100000 => 160,
			0b10110000 => 176,
			0b11000000 => 192,
			0b11010000 => 224,
			0b11100000 => 256,
			_ => return Err(Error::Malformed),
		},
		(Version::II, _) => match value {
			0b00010000 => 8,
			0b00100000 => 16,
			0b00110000 => 24,
			0b01000000 => 32,
			0b01010000 => 40,
			0b01100000 => 48,
			0b01110000 => 56,
			0b10000000 => 64,
			0b10010000 => 80,
			0b10100000 => 96,
			0b10110000 => 112,
			0b11000000 => 128,
			0b11010000 => 144,
			0b11100000 => 160,
			_ => return Err(Error::Malformed),
		},
	} * 1000;

	let value = header[2] & 0b00001100;
	let samples: usize = match version {
		Version::I => match value {
			0b00000000 => 44100,
			0b00000100 => 48000,
			0b00001000 => 32000,
			_ => return Err(Error::Malformed),
		},
		Version::II => match value {
			0b00000000 => 22050,
			0b00000100 => 24000,
			0b00001000 => 16000,
			_ => return Err(Error::Malformed),
		},
	};

	let padding = ((header[2] & 0b00000010) >> 1) as usize;

	let size = match (version, layer) {
		(_, Layer::I) => 12,
		(_, Layer::II) => 144,
		(Version::I, Layer::III) => 144,
		(Version::II, Layer::III) => 72,
	};

	Ok(size * bitrate / samples + padding)
}

fn size(data: &[u8; 4]) -> u32 {
	((data[0] as u32) << 21) + ((data[1] as u32) << 14) + ((data[2] as u32) << 7) + data[3] as u32
}

use crate::error::Error;
use std::{
	io::{Read, Seek, SeekFrom},
	slice,
};

pub fn read<R: Read>(source: &mut R, data: &mut [u8]) -> Result<bool, Error> {
	if source.read(slice::from_mut(&mut data[0]))? < 1 {
		return Ok(false);
	}
	for value in data.iter_mut().skip(1) {
		if source.read(slice::from_mut(value))? < 1 {
			return Err(Error::Malformed);
		}
	}
	Ok(true)
}

pub fn read_u8<R: Read>(source: &mut R) -> Result<u8, Error> {
	let mut data: u8 = 0;
	source.read_exact(slice::from_mut(&mut data))?;
	Ok(data)
}

pub fn read_u16<R: Read>(source: &mut R) -> Result<u16, Error> {
	let mut data: [u8; 2] = [0; 2];
	source.read_exact(&mut data)?;
	let value = u16::from_be_bytes(data);
	Ok(value)
}

pub fn read_u32<R: Read>(source: &mut R) -> Result<u32, Error> {
	let mut data: [u8; 4] = [0; 4];
	source.read_exact(&mut data)?;
	let value = u32::from_le_bytes(data);
	Ok(value)
}

pub fn skip<S: Seek>(source: &mut S, size: u64) -> Result<(), Error> {
	let position = source.stream_position()?;
	if source.seek(SeekFrom::Current(size as i64))? < position + size {
		return Err(Error::Malformed);
	}
	Ok(())
}

use crate::error::Error;
use std::{
	io::{Read, Seek, SeekFrom},
	slice,
};

pub fn read_u8<R: Read>(source: &mut R) -> Result<u8, Error> {
	let mut data: u8 = 0;
	if source.read(slice::from_mut(&mut data))? < 1 {
		return Err(Error::Malformed);
	}
	Ok(data)
}

pub fn read_u16<R: Read>(source: &mut R) -> Result<u16, Error> {
	let mut data: [u8; 2] = [0; 2];
	if source.read(&mut data)? < 2 {
		return Err(Error::Malformed);
	}
	let value = u16::from_be_bytes(data);
	Ok(value)
}

pub fn skip<S: Seek>(source: &mut S, size: u64) -> Result<(), Error> {
	let position = source.seek(SeekFrom::Current(0))?;
	if source.seek(SeekFrom::Current(size as i64))? < position + size {
		return Err(Error::Malformed);
	}
	Ok(())
}

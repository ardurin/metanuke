use super::super::{error::Error, metadata::mp4::delete_metadata};
use std::io::Cursor;

const BASIC: [u8; 16] = [
	0x00, 0x00, 0x00, 0x10, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00, 0x00,
];

#[test]
fn basic() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut reader = Cursor::new(BASIC);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(destination, BASIC);
}

#[test]
fn extended_size() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut source = vec![0; u32::MAX as usize + 1];
	source[3] = 0x01;
	source[4] = b'm';
	source[5] = b'd';
	source[6] = b'a';
	source[7] = b't';
	source[11] = 0x01;
	let mut reader = Cursor::new(&source);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(destination, source);
}

#[test]
fn free() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut reader = Cursor::new([
		0x00, 0x00, 0x00, 0x10, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x0C, b'f', b'r', b'e', b'e', 0x00, 0x00, 0x00, 0x00,
	]);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(destination, BASIC);
}

#[test]
fn skip() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut reader = Cursor::new([
		0x00, 0x00, 0x00, 0x10, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x0C, b's', b'k', b'i', b'p', 0x00, 0x00, 0x00, 0x00,
	]);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(destination, BASIC);
}

#[test]
fn metadata() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut reader = Cursor::new([
		0x00, 0x00, 0x00, 0x10, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x6D, b'm', b'e', b't', b'a', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x21, b'h', b'd', b'l', b'r', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, b'm',
		b'd', b't', b'a', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x1C, b'k', b'e', b'y', b's', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, b'm', b'd', b't', b'a', b'y', b'e', b'a', b'r', 0x00,
		0x00, 0x00, 0x24, b'i', b'l', b's', b't', 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x01,
		0x00, 0x00, 0x00, 0x14, b'd', b'a', b't', b'a', 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
		0x00, b'2', b'0', b'2', b'4',
	]);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(destination, BASIC);
}

#[test]
fn movie_metadata() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut reader = Cursor::new([
		0x00, 0x00, 0x00, 0x10, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x9D, b'm', b'o', b'o', b'v', 0x00, 0x00, 0x00, 0x6D, b'm', b'e',
		b't', b'a', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, b'h', b'd', b'l', b'r', 0x00,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, b'm', b'd', b't', b'a', 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, b'k', b'e',
		b'y', b's', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, b'm',
		b'd', b't', b'a', b'y', b'e', b'a', b'r', 0x00, 0x00, 0x00, 0x24, b'i', b'l', b's', b't',
		0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x14, b'd', b'a', b't',
		b'a', 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, b'2', b'0', b'2', b'4', 0x00, 0x00,
		0x00, 0x28, b'm', b'v', b'e', b'x', 0x00, 0x00, 0x00, 0x20, b't', b'r', b'e', b'x', 0x00,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	]);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		[
			0x00, 0x00, 0x00, 0x10, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x30, b'm', b'o', b'o', b'v', 0x00, 0x00, 0x00, 0x28,
			b'm', b'v', b'e', b'x', 0x00, 0x00, 0x00, 0x20, b't', b'r', b'e', b'x', 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		]
	);
}

#[test]
fn movie_metadata_extended_size() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut source = vec![0; u32::MAX as usize + 1];
	source[3] = 0x01;
	source[4] = b'm';
	source[5] = b'o';
	source[6] = b'o';
	source[7] = b'v';
	source[11] = 0x01;
	source[16] = 0xFF;
	source[17] = 0xFF;
	source[18] = 0xFF;
	source[19] = 0xC8;
	source[20] = b'm';
	source[21] = b'e';
	source[22] = b't';
	source[23] = b'a';
	source[4294967259] = 0x28;
	source[4294967260] = b'm';
	source[4294967261] = b'v';
	source[4294967262] = b'e';
	source[4294967263] = b'x';
	source[4294967267] = 0x20;
	source[4294967268] = b't';
	source[4294967269] = b'r';
	source[4294967270] = b'e';
	source[4294967271] = b'x';
	let mut reader = Cursor::new(&source);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		[
			0x00, 0x00, 0x00, 0x30, b'm', b'o', b'o', b'v', 0x00, 0x00, 0x00, 0x28, b'm', b'v',
			b'e', b'x', 0x00, 0x00, 0x00, 0x20, b't', b'r', b'e', b'x', 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		]
	);
}

#[test]
fn additional_metadata() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut reader = Cursor::new([
		0x00, 0x00, 0x00, 0x10, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x1D, b'm', b'e', b'c', b'o', 0x00, 0x00, 0x00, 0x15, b'm', b'e',
		b'r', b'e', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
	]);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(destination, BASIC);
}

#[test]
fn incorrect_size() {
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	let mut reader = Cursor::new([
		0x00, 0x00, 0x00, 0x14, b'f', b't', b'y', b'p', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00,
		0x00,
	]);
	assert!(matches!(
		delete_metadata(&mut reader, &mut writer),
		Err(Error::Malformed),
	));
}

use super::super::metadata::pdf::delete_metadata;
use std::io::Cursor;

#[test]
fn basic() {
	let mut reader = Cursor::new(
		b"%PDF-1.7\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000101 00000 n \n\
		trailer\n\
		<</Size 4/Root 1 0 R>>\n\
		startxref\n\
		142\n\
        %%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.7\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Kids[3 0 R]/Count 1>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000105 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 4>>\n\
		startxref\n\
		148\n\
		%%EOF",
	);
}

#[test]
fn comments() {
	let mut reader = Cursor::new(
		b"%PDF-1.7\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj %catalog\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj% pages\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj%page \n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000061 00000 n \n\
		0000000117 00000 n \n\
		trailer\n\
		<</Size 4/Root 1 0 R>>\n\
		startxref\n\
		164\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.7\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Kids[3 0 R]/Count 1>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000105 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 4>>\n\
		startxref\n\
		148\n\
		%%EOF",
	);
}

#[test]
fn metadata_dictionary() {
	let mut reader = Cursor::new(
		b"%PDF-1.7\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Metadata>>endobj\n\
		xref\n\
		0 5\n\
        0000000000 65535 f \n\
        0000000009 00000 n \n\
        0000000052 00000 n \n\
        0000000101 00000 n \n\
        0000000142 00000 n \n\
        trailer\n\
        <</Size 5/Root 1 0 R>>\n\
        startxref\n\
        174\n\
        %%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.7\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Kids[3 0 R]/Count 1>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000105 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		148\n\
		%%EOF"
	);
}

#[test]
fn free_object() {
	let mut reader = Cursor::new(
		b"%PDF-1.7\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 5\n\
		0000000003 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000000 00001 f \n\
		0000000142 00000 n \n\
		trailer\n\
		<</Size 5/Root 1 0 R>>\n\
		startxref\n\
		183\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.7\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Kids[3 0 R]/Count 1>>\n\
		endobj\n\
		4 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 3\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		4 1\n\
		0000000105 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		148\n\
		%%EOF"
	);
}

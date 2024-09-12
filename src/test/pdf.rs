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
		<</Root 1 0 R/Size 4>>\n\
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
		<</Root 1 0 R/Size 4>>\n\
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
fn document_information() {
	let mut reader = Cursor::new(
		b"%PDF-1.5\n\
		1 0 obj<</Creator(LibreOffice 20.0)>>endobj\n\
		2 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		3 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		4 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000053 00000 n \n\
		0000000096 00000 n \n\
		0000000145 00000 n \n\
		trailer\n\
		<</Info 1 0 R/Root 2 0 R/Size 5>>\n\
		startxref\n\
		186\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.5\n\
		2 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Pages/Kids[3 0 R]/Count 1>>\n\
		endobj\n\
		4 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 1\n\
		0000000000 65535 f \n\
		2 3\n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000105 00000 n \n\
		trailer\n\
		<</Size 5/Root 2 0 R>>\n\
		startxref\n\
		148\n\
		%%EOF",
	);
}

#[test]
fn document_information_deprecated() {
	let mut reader = Cursor::new(
		b"%PDF-2.0\n\
		1 0 obj<</Creator(LibreOffice 20.0)>>endobj\n\
		2 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		3 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		4 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000053 00000 n \n\
		0000000096 00000 n \n\
		0000000145 00000 n \n\
		trailer\n\
		<</Info 1 0 R/Root 2 0 R/Size 5>>\n\
		startxref\n\
		186\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-2.0\n\
		2 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Pages/Kids[3 0 R]/Count 1>>\n\
		endobj\n\
		4 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 1\n\
		0000000000 65535 f \n\
		2 3\n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000105 00000 n \n\
		trailer\n\
		<</Size 5/Root 2 0 R>>\n\
		startxref\n\
		148\n\
		%%EOF",
	);
}

#[test]
fn document_metadata() {
	let mut reader = Cursor::new(
		b"%PDF-1.5\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R/Metadata 4 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Metadata/Subtype/XML/Length 258>>stream\n\
		<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		<xmpmeta xmlns:x=\"adobe:ns:meta/\">\n\
		<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n\
		<rdf:Description rdf:about=\"\">\n\
		<pdf:Producer>LibreOffice 20.0</pdf:Producer>\n\
		</rdf:Description>\n\
		</rdf:RDF>\n\
		</xmpmeta>\n\
		endstream\n\
		endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000067 00000 n \n\
		0000000116 00000 n \n\
		0000000157 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		487\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.5\n\
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
		%%EOF",
	);
}

#[test]
fn object_metadata() {
	let mut reader = Cursor::new(
		b"%PDF-1.8\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R/Metadata 4 0 R>>endobj\n\
		4 0 obj<</Type/Metadata/Subtype/XML/Length 258>>stream\n\
		<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		<xmpmeta xmlns:x=\"adobe:ns:meta/\">\n\
		<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n\
		<rdf:Description rdf:about=\"\">\n\
		<pdf:Producer>LibreOffice 20.0</pdf:Producer>\n\
		</rdf:Description>\n\
		</rdf:RDF>\n\
		</xmpmeta>\n\
		endstream\n\
		endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000101 00000 n \n\
		0000000157 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		487\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.8\n\
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
fn detached_metadata() {
	let mut reader = Cursor::new(
		b"%PDF-1.2\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Metadata/Subtype/XML/Length 258>>stream\n\
		<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		<xmpmeta xmlns:x=\"adobe:ns:meta/\">\n\
		<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n\
		<rdf:Description rdf:about=\"\">\n\
		<pdf:Producer>LibreOffice 20.0</pdf:Producer>\n\
		</rdf:Description>\n\
		</rdf:RDF>\n\
		</xmpmeta>\n\
		endstream\n\
		endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000101 00000 n \n\
		0000000142 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		472\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.2\n\
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
fn signature() {
	let mut reader = Cursor::new(
		b"%PDF-1.2\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Sig/Filter/Adobe.PPKLite/SubFilter/adbe.pkcs7.detached/Contents<E1A9DE5DC7F97CC18CADE55D04EA0B3DD52AC4F0>>>endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000101 00000 n \n\
		0000000142 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		271\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.2\n\
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
		%%EOF",
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
		<</Root 1 0 R/Size 5>>\n\
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

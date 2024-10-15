use super::super::metadata::pdf::delete_metadata;
use std::io::Cursor;

#[test]
fn basic() {
	let mut reader = Cursor::new(
		b"%PDF-1.0\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000123 00000 n \n\
		trailer<</Root 1 0 R/Size 4>>\n\
		startxref\n\
		164\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.0\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 4>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn comments() {
	let mut reader = Cursor::new(
		b"%PDF-1.1\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj %catalog\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj% pages\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj%page \n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000061 00000 n \n\
		0000000139 00000 n \n\
		trailer<</Root 1 0 R/Size 4>>\n\
		startxref\n\
		186\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.1\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 4>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn extra() {
	let mut reader = Cursor::new(
		b"%PDF-1.2\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000123 00000 n \n\
		trailer<</Root 1 0 R/Size 4>>????\n\
		startxref\n\
		164\n\
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
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 4>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn cross_reference_stream() {
	let mut reader = Cursor::new(
		b"%PDF-1.5\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/XRef/Index[1 3]/W[1 1 1]/Root 1 0 R/Size 4/Length 9>>stream\n\
		\x01\x09\x00\x01\x34\x00\x01\x7B\x00endstream\n\
		endobj\n\
		startxref\n\
		164\n\
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
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		4 0 obj\n\
		<</Type/XRef/Root 1 0 R/Size 5/W[1 4 2]/Index[1 4]/Length 28>>stream\n\
		\x01\x00\x00\x00\x09\x00\x00\x01\x00\x00\x00\x36\x00\x00\x01\x00\x00\x00\x7F\x00\x00\x01\x00\x00\x00\xAA\x00\x00\n\
		endstream \n\
		endobj\n\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn object_stream() {
	let mut reader = Cursor::new(
		b"%PDF-1.5\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		3 0 obj<</Type/ObjStm/First 3/N 1/Length 30>>stream\n\
		4 0<</Type/Page/Parent 2 0 R>>endstream\n\
		endobj\n\
		5 0 obj<</Type/XRef/Index[1 4]/W[1 1 1]/Root 1 0 R/Size 5/Length 12>>stream\n\
		\x01\x09\x00\x01\x34\x00\x01\x7B\x00\x02\x03\x03endstream\n\
		endobj\n\
		startxref\n\
		222\n\
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
		<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		4 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		5 0 obj\n\
		<</Type/XRef/Root 1 0 R/Size 6/W[1 4 2]/Index[1 2 4 2]/Length 28>>stream\n\
		\x01\x00\x00\x00\x09\x00\x00\x01\x00\x00\x00\x36\x00\x00\x01\x00\x00\x00\x7F\x00\x00\x01\x00\x00\x00\xAA\x00\x00\n\
		endstream \n\
		endobj\n\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn document_information() {
	let mut reader = Cursor::new(
		b"%PDF-1.0\n\
		1 0 obj<</Creator(LibreOffice 20.0)>>endobj\n\
		2 0 obj<</Type/Catalog/Pages 3 0 R>>endobj\n\
		3 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		4 0 obj<</Type/Page/Parent 3 0 R>>endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000053 00000 n \n\
		0000000096 00000 n \n\
		0000000167 00000 n \n\
		trailer<</Info 1 0 R/Root 2 0 R/Size 5>>\n\
		startxref\n\
		208\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.0\n\
		2 0 obj\n\
		<</Type/Catalog/Pages 3 0 R>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		4 0 obj\n\
		<</Type/Page/Parent 3 0 R>>\n\
		endobj\n\
		xref\n\
		0 1\n\
		0000000000 65535 f \n\
		2 3\n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Size 5/Root 2 0 R>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn document_information_deprecated() {
	let mut reader = Cursor::new(
		b"%PDF-2.0\n\
		1 0 obj<</Creator(LibreOffice 20.0)>>endobj\n\
		2 0 obj<</Type/Catalog/Pages 3 0 R>>endobj\n\
		3 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		4 0 obj<</Type/Page/Parent 3 0 R>>endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000053 00000 n \n\
		0000000096 00000 n \n\
		0000000167 00000 n \n\
		trailer<</Info 1 0 R/Root 2 0 R/Size 5>>\n\
		startxref\n\
		208\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-2.0\n\
		2 0 obj\n\
		<</Type/Catalog/Pages 3 0 R>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		4 0 obj\n\
		<</Type/Page/Parent 3 0 R>>\n\
		endobj\n\
		xref\n\
		0 1\n\
		0000000000 65535 f \n\
		2 3\n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Size 5/Root 2 0 R>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn document_metadata() {
	let mut reader = Cursor::new(
		b"%PDF-1.6\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R/Metadata 4 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
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
		0000000138 00000 n \n\
		0000000179 00000 n \n\
		trailer<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		509\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.6\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn object_metadata() {
	let mut reader = Cursor::new(
		b"%PDF-1.7\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
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
		0000000123 00000 n \n\
		0000000179 00000 n \n\
		trailer<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		509\n\
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
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		170\n\
		%%EOF"
	);
}

#[test]
fn detached_metadata() {
	let mut reader = Cursor::new(
		b"%PDF-1.7\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
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
		0000000123 00000 n \n\
		0000000164 00000 n \n\
		trailer<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		494\n\
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
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		170\n\
		%%EOF"
	);
}

#[test]
fn signature() {
	let mut reader = Cursor::new(
		b"%PDF-1.2\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Sig/Filter/Adobe.PPKLite/SubFilter/adbe.pkcs7.detached/Contents<E1A9DE5DC7F97CC18CADE55D04EA0B3DD52AC4F0>>>endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000123 00000 n \n\
		0000000164 00000 n \n\
		trailer<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		293\n\
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
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

#[test]
fn free_object() {
	let mut reader = Cursor::new(
		b"%PDF-1.4\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 5\n\
		0000000003 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000000 00001 f \n\
		0000000164 00000 n \n\
		trailer<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		205\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.4\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>\n\
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
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		170\n\
		%%EOF"
	);
}

#[test]
fn unreferenced_object() {
	let mut reader = Cursor::new(
		b"%PDF-1.4\n\
		1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n\
		2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj\n\
		3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		4 0 obj<</Type/Page/Parent 2 0 R>>endobj\n\
		xref\n\
		0 5\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000052 00000 n \n\
		0000000123 00000 n \n\
		0000000164 00000 n \n\
		trailer<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		205\n\
		%%EOF",
	);
	let mut destination = Vec::new();
	let mut writer = Cursor::new(&mut destination);
	assert!(matches!(delete_metadata(&mut reader, &mut writer), Ok(())));
	assert_eq!(
		destination,
		b"%PDF-1.4\n\
		1 0 obj\n\
		<</Type/Catalog/Pages 2 0 R>>\n\
		endobj\n\
		2 0 obj\n\
		<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>\n\
		endobj\n\
		3 0 obj\n\
		<</Type/Page/Parent 2 0 R>>\n\
		endobj\n\
		xref\n\
		0 4\n\
		0000000000 65535 f \n\
		0000000009 00000 n \n\
		0000000054 00000 n \n\
		0000000127 00000 n \n\
		trailer\n\
		<</Root 1 0 R/Size 5>>\n\
		startxref\n\
		170\n\
		%%EOF",
	);
}

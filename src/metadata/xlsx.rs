use crate::Error;
use std::io::{Read, Seek, Write};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

pub fn delete_metadata<R: Read + Seek, W: Write + Seek>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	let mut source = ZipArchive::new(source)?;
	let mut destination = ZipWriter::new(destination);
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
	for i in 0..source.len() {
		let entry = source.by_index(i)?;
		let name = entry.name();
		match name {
			"[Content_Types].xml" | "_rels/.rels" => {
				destination.raw_copy_file(entry)?;
			}
			"docProps/app.xml" => {
				destination.start_file(name, options)?;
				destination.write(br#"<?xml version="1.0" encoding="UTF-8"?><Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes"></Properties>"#)?;
			}
			"docProps/core.xml" => {
				destination.start_file(name, options)?;
				destination.write(br#"<?xml version="1.0" encoding="UTF-8"?><cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"></cp:coreProperties>"#)?;
			}
			"docProps/custom.xml" => {
				destination.start_file(name, options)?;
				destination.write(br#"<?xml version="1.0" encoding="UTF-8"?><Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/custom-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes"></Properties>"#)?;
			}
			name if name.starts_with("xl/") || name.starts_with("customXml/") => {
				destination.raw_copy_file(entry)?;
			}
			_ => {}
		}
	}
	destination.finish()?;
	Ok(())
}

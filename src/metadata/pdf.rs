use crate::error::Error;
use lopdf::{Dictionary, Document, Object, Reader};
use std::io::{Read, Write};

pub fn delete_metadata<R: Read, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	let mut data = Vec::new();
	source.read_to_end(&mut data)?;
	let mut document = Reader {
		buffer: &data,
		document: Document::new(),
	}
	.read(Some(evaluate))?;
	if document.is_encrypted() {
		// lopdf does not correctly parse PDFs with encrypted object streams
		return Err(Error::Encrypted);
	}
	document.trailer.remove(b"DocChecksum");
	document.trailer.remove(b"Info");
	if let Ok(catalog) = document.catalog_mut() {
		catalog.remove(b"Lang");
		catalog.remove(b"Legal");
		catalog.remove(b"Perms");
		catalog.remove(b"SpiderInfo");
	}
	document.prune_objects();
	document.save_to(destination)?;
	Ok(())
}

fn evaluate(identifier: (u32, u16), object: &mut Object) -> Option<((u32, u16), Object)> {
	match object.type_name() {
		Ok("DocTimeStamp") | Ok("Metadata") | Ok("Sig") => None,
		_ => {
			if let Ok(dictionary) = object.as_dict_mut() {
				delete_entries(dictionary);
			} else if let Ok(stream) = object.as_stream_mut() {
				delete_entries(&mut stream.dict);
			}
			Some((identifier, object.clone()))
		}
	}
}

fn delete_entries(dictionary: &mut Dictionary) {
	dictionary.remove(b"LastModified");
	dictionary.remove(b"Metadata");
	dictionary.remove(b"PieceInfo");
}

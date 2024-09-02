use crate::error::Error;
use lopdf::Document;
use std::io::{Read, Write};

pub fn delete_metadata<R: Read, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	let mut data = Vec::new();
	source.read_to_end(&mut data)?;
	let mut delete = vec![];
	let mut document = Document::load_from(data.as_slice())?;
	for (identifier, object) in document.objects.iter() {
		if let Ok("Metadata") = object.type_name() {
			delete.push(identifier.clone());
		}
	}
	for identifier in delete {
		document.delete_object(identifier);
	}
	document.save_to(destination)?;
	Ok(())
}

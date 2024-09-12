use crate::error::Error;
use lopdf::{Document, Object};
use std::io::{Read, Write};

pub fn delete_metadata<R: Read, W: Write>(
	source: &mut R,
	destination: &mut W,
) -> Result<(), Error> {
	let mut data = Vec::new();
	source.read_to_end(&mut data)?;
	let mut delete = vec![];
	let mut document = Document::load_from(data.as_slice())?;
	if let Ok(Object::Reference(reference)) = document.trailer.get(b"Info") {
		delete.push(*reference);
	}
	// Document::delete_object does not check the trailer for references to that object
	document.trailer.remove(b"Info");
	for (reference, object) in document.objects.iter_mut() {
		match object.type_name() {
			Ok("DocTimeStamp") | Ok("Metadata") | Ok("Sig") => {
				delete.push(*reference);
			}
			_ => {
				if let Ok(dictionary) = object.as_dict_mut() {
					dictionary.remove(b"LastModified");
					dictionary.remove(b"PieceInfo");
				}
			}
		}
	}
	for identifier in delete {
		document.delete_object(identifier);
	}
	document.save_to(destination)?;
	Ok(())
}

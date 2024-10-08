use crate::{file::*, identify::identify, options::Options};
use error::Error;
use std::{
	fs::File,
	io::{BufReader, BufWriter},
};

mod error;
mod file;
mod identify;
mod metadata;
mod options;
#[cfg(test)]
mod test;
mod util;

fn main() -> Result<(), Error> {
	let Ok(options) = Options::parse() else {
		println!("Usage: metanuke [-o <file>|--replace] <file>");
		return Ok(());
	};
	let delete_metadata = identify(&options.source)?;
	let (file, temporary) = create_unique(get_directory(&options.destination))?;
	let mut reader = BufReader::new(File::open(&options.source)?);
	let mut writer = BufWriter::new(file);
	delete_metadata(&mut reader, &mut writer)?;
	let destination = options
		.destination
		.unwrap_or_else(|| create_from_template(&options.source));
	temporary.persist(destination)?;
	Ok(())
}

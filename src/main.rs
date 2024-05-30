use crate::{file::*, identify::identify, options::Options};
use error::Error;
use std::{
	fs::{rename, File},
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
		println!("Usage: mwipe [-o <file>|--replace] <file>");
		return Ok(());
	};
	let delete_metadata = identify(&options.source)?;
	let temporary = create_unique(get_directory(&options.destination))?;
	let mut reader = BufReader::new(File::open(&options.source)?);
	let mut writer = BufWriter::new(temporary.file);
	delete_metadata(&mut reader, &mut writer)?;
	let destination = options
		.destination
		.unwrap_or_else(|| create_from_template(&options.source));
	rename(temporary.path, destination)?;
	Ok(())
}

use fastrand::alphanumeric;
use std::{
	borrow::Cow,
	env::current_dir,
	ffi::{OsStr, OsString},
	fs::{File, OpenOptions},
	io::ErrorKind,
	iter::repeat_with,
	path::{Path, PathBuf},
};

pub struct Temporary {
	pub file: File,
	pub path: PathBuf,
}

pub fn create_unique<P: AsRef<Path>>(directory: P) -> Result<Temporary, std::io::Error> {
	let mut name = String::with_capacity(10);
	loop {
		name.clear();
		let random: String = repeat_with(alphanumeric).take(5).collect();
		name.push_str(".");
		name.push_str(random.as_str());
		name.push_str(".tmp");
		let path = directory.as_ref().join(&name);
		match OpenOptions::new().create_new(true).write(true).open(&path) {
			Ok(file) => {
				return Ok(Temporary { path, file });
			}
			Err(error) => {
				if error.kind() != ErrorKind::AlreadyExists {
					return Err(error);
				}
			}
		}
	}
}

pub fn create_from_template<P: AsRef<Path>>(template: P) -> PathBuf {
	let template = template.as_ref();
	let extension = template.extension().unwrap_or(OsStr::new(""));
	let stem = template.file_stem().unwrap_or(OsStr::new(""));
	let mut name = OsString::new();
	name.push(stem);
	name.push(".");
	name.push(extension);
	let mut i = 1;
	loop {
		if OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(&name)
			.is_ok()
		{
			return PathBuf::from(name);
		}
		i += 1;
		name.clear();
		name.push(stem);
		name.push("-");
		name.push(i.to_string());
		name.push(".");
		name.push(extension);
	}
}

pub fn get_directory(file: &Option<PathBuf>) -> Cow<Path> {
	match file {
		Some(path) => match path.parent() {
			Some(path) => {
				if path.as_os_str().len() == 0 {
					Cow::Owned(current_dir().unwrap())
				} else {
					Cow::Borrowed(path)
				}
			}
			None => Cow::Borrowed(path),
		},
		None => Cow::Owned(current_dir().unwrap()),
	}
}

use fastrand::alphanumeric;
use std::{
	borrow::Cow,
	env::current_dir,
	ffi::OsString,
	fs::{remove_file, rename, File, OpenOptions},
	io::{self, ErrorKind},
	iter::repeat_with,
	path::{Path, PathBuf},
};

pub struct Temporary {
	delete: bool,
	path: PathBuf,
}

impl Temporary {
	pub fn persist<P: AsRef<Path>>(mut self, path: P) -> io::Result<()> {
		rename(&self.path, path)?;
		self.delete = false;
		Ok(())
	}
}

impl Drop for Temporary {
	fn drop(&mut self) {
		if self.delete {
			let _ = remove_file(&self.path);
		}
	}
}

pub fn create_unique<P: AsRef<Path>>(directory: P) -> io::Result<(File, Temporary)> {
	let mut name = String::with_capacity(10);
	loop {
		let random: String = repeat_with(alphanumeric).take(5).collect();
		name.push_str(".");
		name.push_str(random.as_str());
		name.push_str(".tmp");
		let path = directory.as_ref().join(&name);
		match OpenOptions::new().create_new(true).write(true).open(&path) {
			Ok(descriptor) => {
				return Ok((descriptor, Temporary { delete: true, path }));
			}
			Err(error) => {
				if error.kind() != ErrorKind::AlreadyExists {
					return Err(error);
				}
			}
		}
		name.clear();
	}
}

pub fn create_from_template<P: AsRef<Path>>(template: P) -> PathBuf {
	let template = template.as_ref();
	let extension = template.extension().unwrap();
	let stem = template.file_stem().unwrap();
	let mut name = OsString::new();
	name.push(stem);
	name.push(".");
	name.push(extension);
	let mut suffix = 1;
	loop {
		if OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(&name)
			.is_ok()
		{
			return PathBuf::from(name);
		}
		suffix += 1;
		name.clear();
		name.push(stem);
		name.push("-");
		name.push(suffix.to_string());
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

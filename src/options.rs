use std::{env::args, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Options {
	pub destination: Option<PathBuf>,
	pub source: PathBuf,
}

impl Options {
	pub fn parse() -> Result<Options, ()> {
		let mut arguments = args();
		parse(&mut arguments)
	}
}

macro_rules! validate {
	($path:expr) => {
		if $path.len() > 0 {
			Ok(PathBuf::from($path))
		} else {
			Err(())
		}
	};
}

fn parse<I: Iterator<Item = String>>(arguments: &mut I) -> Result<Options, ()> {
	let mut destination: Option<PathBuf> = None;
	let mut replace = false;
	arguments.next();
	loop {
		if let Some(argument) = arguments.next() {
			if argument.starts_with('-') {
				match argument.as_str() {
					"--replace" => {
						if destination.is_some() {
							return Err(());
						}
						replace = true;
					}
					"-o" => {
						if replace {
							return Err(());
						}
						match arguments.next() {
							Some(path) => destination = Some(validate!(path)?),
							_ => return Err(()),
						}
					}
					"--" => match arguments.next() {
						Some(path) => {
							let path = validate!(path)?;
							return Ok(Options {
								source: path.clone(),
								destination: if replace { Some(path) } else { destination },
							});
						}
						_ => return Err(()),
					},
					_ => {
						return Err(());
					}
				}
			} else {
				let path = validate!(argument)?;
				return Ok(Options {
					source: path.clone(),
					destination: if replace { Some(path) } else { destination },
				});
			}
		} else {
			return Err(());
		}
	}
}

#[cfg(test)]
mod test {
	use super::{parse, Options};
	use std::path::PathBuf;

	#[test]
	fn minimal() {
		let mut arguments = vec!["".into(), "in.png".into()].into_iter();
		assert_eq!(
			parse(&mut arguments),
			Ok(Options {
				destination: None,
				source: PathBuf::from("in.png"),
			})
		);
	}

	#[test]
	fn destination() {
		let mut arguments =
			vec!["".into(), "-o".into(), "out.png".into(), "in.png".into()].into_iter();
		assert_eq!(
			parse(&mut arguments),
			Ok(Options {
				destination: Some(PathBuf::from("out.png")),
				source: PathBuf::from("in.png"),
			})
		);
	}

	#[test]
	fn replace() {
		let mut arguments = vec!["".into(), "--replace".into(), "in.png".into()].into_iter();
		assert_eq!(
			parse(&mut arguments),
			Ok(Options {
				destination: Some(PathBuf::from("in.png")),
				source: PathBuf::from("in.png"),
			})
		);
	}

	#[test]
	fn destination_and_replace() {
		let mut arguments = vec![
			"".into(),
			"-o".into(),
			"out.png".into(),
			"--replace".into(),
			"in.png".into(),
		]
		.into_iter();
		assert!(matches!(parse(&mut arguments), Err(_)));
	}

	#[test]
	fn dash() {
		let mut arguments = vec!["".into(), "--".into(), "-.png".into()].into_iter();
		assert_eq!(
			parse(&mut arguments),
			Ok(Options {
				destination: None,
				source: PathBuf::from("-.png"),
			})
		);
	}

	#[test]
	fn dash_replace() {
		let mut arguments = vec![
			"".into(),
			"-o".into(),
			"out.png".into(),
			"--".into(),
			"-.png".into(),
		]
		.into_iter();
		assert_eq!(
			parse(&mut arguments),
			Ok(Options {
				destination: Some(PathBuf::from("out.png")),
				source: PathBuf::from("-.png"),
			})
		);
	}

	#[test]
	fn empty_source() {
		let mut arguments = vec!["".into(), "".into()].into_iter();
		assert!(matches!(parse(&mut arguments), Err(_)));
	}

	#[test]
	fn empty_destination() {
		let mut arguments = vec!["".into(), "-o".into(), "".into(), "in.png".into()].into_iter();
		assert!(matches!(parse(&mut arguments), Err(_)));
	}

	#[test]
	fn unknown_option() {
		let mut arguments = vec!["".into(), "-replace".into(), "in.png".into()].into_iter();
		assert!(matches!(parse(&mut arguments), Err(_)));
	}

	#[test]
	fn unknown_option_value() {
		let mut arguments =
			vec!["".into(), "-O".into(), "out.png".into(), "in.png".into()].into_iter();
		assert!(matches!(parse(&mut arguments), Err(_)));
	}

	#[test]
	fn source_missing() {
		let mut arguments = vec!["".into()].into_iter();
		assert!(matches!(parse(&mut arguments), Err(_)));
	}
}

#![cfg(all(windows, target_arch = "x86_64"))]

// TODO: Have proper errors and print nicer messages using fav error handling library.
// Add support for other arches.
// Also completely rewrite this so that the code is sane.

use minwinsdk::{known_folder, FOLDERID_UserProgramFilesCommon};
use std::{
	fs,
	io::{self, Write},
	str,
};

const MACHINES: &[&str] = &["x64", "x86", "arm64", "arm"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Unpacking libraries."); // Lies.
	let mut libs = known_folder(FOLDERID_UserProgramFilesCommon).unwrap();
	libs.push(r"Minimal Windows SDK");

	for machine in MACHINES {
		libs.push(machine);
		fs::create_dir_all(&libs)?;
		libs.pop();
	}

	for (name, data) in minwinsdk::LIBS {
		let mut path = libs.clone();
		path.push(name);
		let mut f = fs::File::create(path)?;
		f.write_all(data)?;
	}

	println!("Setting global environment.");

	// Update `LIB` environment variable, globally.
	let key = minwinsdk::RegKey::open("Environment")?;
	let value = match key.query_value("LIB") {
		Ok(value) => value,
		Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
		e => e?,
	};

	// TODO: Should detect if path is already in `LIB`.
	libs.push("x64");
	let mut s: String = libs.to_string_lossy().into();
	libs.pop();
	s.push(';');
	s.push_str(&value);
	key.set_value("LIB", &s)?;

	// Help users to find where it installed.
	key.set_value("MIN_WIN_SDK", &libs.to_string_lossy())?;

	// broadcast `WM_SETTINGCHANGE`.
	minwinsdk::broadcast_changes()?;

	println!("Successfully installed libs to:");
	println!("\t{}", libs.display());
	println!("To start using these libs you may need to restart your current shell.");
	println!("But you might want to install rustup if you haven't already.");

	Ok(())
}

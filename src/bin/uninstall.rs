use minwinsdk::{known_folder, FOLDERID_UserProgramFilesCommon};
use std::{fs, io, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Uninstalling...");
	let mut libs = known_folder(FOLDERID_UserProgramFilesCommon).unwrap();
	libs.push(r"Minimal Windows SDK");

	println!("Removing values from global environment.");

	let key = minwinsdk::RegKey::open("Environment")?;
	let lib_env = match key.query_value("LIB") {
		Ok(value) => value,
		Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
		e => e?,
	};

	let sdk_env = match key.query_value("MIN_WIN_SDK") {
		Ok(value) => Some(value),
		Err(e) if e.kind() == io::ErrorKind::NotFound => None,
		Err(e) => Err(e)?,
	};
	if let Some(mut sdk) = sdk_env {
		sdk.truncate(sdk.trim_end_matches('\0').len());
		key.delete_value("MIN_WIN_SDK")?;
	}

	let mut new_libs = String::new();
	libs.push("x64");
	for lib in lib_env.split(';') {
		if !lib.is_empty() && libs != Path::new(lib) {
			new_libs.push_str(lib);
			new_libs.push(';');
		}
	}
	new_libs.truncate(new_libs.trim_end_matches(';').len());
	if new_libs.trim().is_empty() {
		key.delete_value("LIB")?;
	} else {
		key.set_value("LIB", &new_libs)?;
	}
	libs.pop();

	// broadcast `WM_SETTINGCHANGE`.
	minwinsdk::broadcast_changes()?;

	println!("Removing files from: {}", libs.display());
	fs::remove_dir_all(&libs)?;

	Ok(())
}

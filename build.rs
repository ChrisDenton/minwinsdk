use std::{
	fs,
	io::{self, Write},
	path::PathBuf,
	process::Command,
};

struct Lib {
	ident: String,
	name: String,
	machine: String,
	path: String,
}

fn make_libs(machine: &str, libraries: &mut Vec<Lib>) -> io::Result<()> {
	let mut out_path = PathBuf::from("libs");
	out_path.push(machine);

	fs::create_dir_all(&out_path)?;

	for entry in fs::read_dir("defs")? {
		let def_path = entry?.path();
		let name = def_path.file_stem().unwrap().to_str().unwrap();
		let output = Command::new("lld-link.exe")
			.args(&[
				&format!("/def:{}", def_path.display()),
				&format!("/out:{}/{}.lib", out_path.display(), name),
				&format!("/machine:{}", machine),
			])
			.output()?;
		if not(output.status.success()) {
			let msg = String::from_utf8_lossy(&output.stderr);
			panic!(r"{}\{}: {}", machine, name, &msg);
		}

		let ident = name.to_uppercase().replace("-", "_").replace(".", "_");
		let ident = machine.to_uppercase() + "_" + &ident;

		libraries.push(Lib {
			ident,
			name: name.to_string(),
			machine: machine.to_string(),
			path: out_path.display().to_string(),
		});
	}

	Ok(())
}

fn main() -> io::Result<()> {
	println!("cargo:rerun-if-changed=defs");

	let mut f = fs::File::create(r"src\imports.rs")?;

	let mut libs = Vec::new();
	make_libs("x64", &mut libs)?;
	make_libs("x86", &mut libs)?;
	make_libs("arm", &mut libs)?;
	make_libs("arm64", &mut libs)?;

	writeln!(f, "pub static LIBS: &[(&str, &[u8])] = &[")?;
	for lib in libs.iter() {
		writeln!(
			f,
			"\t(r#\"{}/{}.lib\"#, {}),",
			lib.machine, lib.name, lib.ident
		)?;
	}
	writeln!(f, "];")?;
	for lib in libs.iter() {
		writeln!(
			f,
			"static {}: &[u8] = include_bytes!(r#\"../{}/{}.lib\"#);",
			lib.ident, lib.path, lib.name
		)?;
	}
	Ok(())
}

fn not(b: bool) -> bool {
	!b
}

use std::fs;
use std::path::Path;

pub const ROLLEN: &[&str] = &[
	"lieferant_neu",
	"netzbetreiber",
	"lieferant_alt",
	"messstellenbetreiber",
	"bilanzkreisverantwortlicher",
	"marktgebietsverantwortlicher",
];

pub fn run(path: &str) {
	let base = Path::new(path);

	for rolle in ROLLEN {
		let rolle_dir = base.join(rolle);
		fs::create_dir_all(rolle_dir.join("inbox")).expect("inbox erstellen");
		fs::create_dir_all(rolle_dir.join("outbox")).expect("outbox erstellen");
		let state_path = rolle_dir.join("state.json");
		if !state_path.exists() {
			fs::write(&state_path, "{}\n").expect("state.json schreiben");
		}
	}

	fs::create_dir_all(base.join("log")).expect("log erstellen");

	println!("Markt initialisiert in {}/", path);
	for rolle in ROLLEN {
		println!(
			"  {rolle}/inbox/ {rolle}/outbox/ {rolle}/state.json"
		);
	}
	println!("  log/");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn init_creates_all_directories() {
		let dir = std::env::temp_dir().join("mako-cli-test-init");
		let _ = std::fs::remove_dir_all(&dir);
		run(dir.to_str().unwrap());
		for rolle in ROLLEN {
			assert!(dir.join(rolle).join("inbox").is_dir());
			assert!(dir.join(rolle).join("outbox").is_dir());
			assert!(dir.join(rolle).join("state.json").exists());
		}
		assert!(dir.join("log").is_dir());
		std::fs::remove_dir_all(&dir).ok();
	}
}

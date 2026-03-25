use std::collections::HashMap;
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

/// Generate a deterministic MP-ID for the given index (matches mako-testdata convention).
pub fn mp_id_for_index(index: usize) -> String {
	format!("{:013}", 9900000000000u64 + index as u64)
}

pub fn run(path: &str) {
	let base = Path::new(path);

	let mut rollen_map: HashMap<String, String> = HashMap::new();

	for (i, rolle) in ROLLEN.iter().enumerate() {
		let rolle_dir = base.join(rolle);
		fs::create_dir_all(rolle_dir.join("inbox")).expect("inbox erstellen");
		fs::create_dir_all(rolle_dir.join("outbox")).expect("outbox erstellen");
		let state_path = rolle_dir.join("state.json");
		if !state_path.exists() {
			fs::write(&state_path, "{}\n").expect("state.json schreiben");
		}
		rollen_map.insert(mp_id_for_index(i), rolle.to_string());
	}

	// Write rollen.json mapping MP-IDs to directory names
	let rollen_json = serde_json::to_string_pretty(&rollen_map).expect("serialize rollen");
	fs::write(base.join("rollen.json"), rollen_json).expect("rollen.json schreiben");

	fs::create_dir_all(base.join("log")).expect("log erstellen");

	println!("Markt initialisiert in {}/", path);
	for (i, rolle) in ROLLEN.iter().enumerate() {
		println!(
			"  {rolle}/inbox/ {rolle}/outbox/ {rolle}/state.json  (MP-ID: {})",
			mp_id_for_index(i)
		);
	}
	println!("  rollen.json");
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
		assert!(dir.join("rollen.json").exists());

		// Verify rollen.json content
		let content = std::fs::read_to_string(dir.join("rollen.json")).unwrap();
		let map: HashMap<String, String> = serde_json::from_str(&content).unwrap();
		assert_eq!(map.len(), ROLLEN.len());
		assert_eq!(map.get("9900000000000").unwrap(), "lieferant_neu");
		assert_eq!(map.get("9900000000001").unwrap(), "netzbetreiber");

		std::fs::remove_dir_all(&dir).ok();
	}
}

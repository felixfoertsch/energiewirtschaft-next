use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// All market roles instantiated in a fresh markt directory.
///
/// The set mirrors `mako_types::rolle::MarktRolle` minus abstract roles
/// (`Lieferant`, `Rechnungsersteller`, `Rechnungsempfaenger`) — those are
/// realised by concrete roles depending on context.
///
/// Order matters: the index drives the deterministic MP-ID assignment, so do
/// not reorder existing entries — append new ones at the end.
pub const ROLLEN: &[&str] = &[
	// indices 0-5: original set — append-only contract for MP-ID stability.
	"lieferant_neu",                 // 0
	"netzbetreiber",                 // 1
	"lieferant_alt",                 // 2
	"messstellenbetreiber",          // 3
	"bilanzkreisverantwortlicher",   // 4
	"marktgebietsverantwortlicher",  // 5
	// indices 6+: extended set for full Marktrollen-Abdeckung.
	"lieferant_ersatz_grundversorgung",
	"netzbetreiber_alt",
	"netzbetreiber_neu",
	"messstellenbetreiber_alt",
	"messstellenbetreiber_neu",
	"grundzustaendiger_messstellenbetreiber",
	"messdienstleister",
	"uebertragungsnetzbetreiber",
	"bilanzkoordinator",
	"anschlussnetzbetreiber",
	"anfordernder_netzbetreiber",
	"wettbewerblicher_messstellenbetreiber",
	"einsatzverantwortlicher",
	"betreiber_technische_ressource",
	"data_provider",
	"betreiber_erzeugungsanlage",
	"direktvermarkter",
	"energieserviceanbieter",
	"aggregator",
	"ladepunktbetreiber",
	"registerbetreiber_hknr",
	"fernleitungsnetzbetreiber",
	"transportkunde",
	"kapazitaetsnutzer",
	"speicherstellenbetreiber",
	"einspeisenetzbetreiber",
	"ausspeisenetzbetreiber",
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

	let rollen_json = serde_json::to_string_pretty(&rollen_map).expect("serialize rollen");
	fs::write(base.join("rollen.json"), rollen_json).expect("rollen.json schreiben");

	fs::create_dir_all(base.join("log")).expect("log erstellen");

	println!("Markt initialisiert in {}/  ({} Rollen)", path, ROLLEN.len());
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

		let content = std::fs::read_to_string(dir.join("rollen.json")).unwrap();
		let map: HashMap<String, String> = serde_json::from_str(&content).unwrap();
		assert_eq!(map.len(), ROLLEN.len());
		// Stable mapping — original indices 0..5 must not move so existing
		// tests/fixtures keep working.
		assert_eq!(map.get("9900000000000").unwrap(), "lieferant_neu");
		assert_eq!(map.get("9900000000001").unwrap(), "netzbetreiber");
		assert_eq!(map.get("9900000000002").unwrap(), "lieferant_alt");
		assert_eq!(map.get("9900000000003").unwrap(), "messstellenbetreiber");

		std::fs::remove_dir_all(&dir).ok();
	}

	#[test]
	fn rollen_contains_redispatch_set() {
		// User-Anforderung: Einsatzverantwortlicher und Betreiber Technische Ressource
		// müssen vorhanden sein.
		assert!(ROLLEN.contains(&"einsatzverantwortlicher"));
		assert!(ROLLEN.contains(&"betreiber_technische_ressource"));
		assert!(ROLLEN.contains(&"data_provider"));
		assert!(ROLLEN.contains(&"anschlussnetzbetreiber"));
	}

	#[test]
	fn rollen_contains_msb_variants() {
		// WiM braucht alt/neu/grundzuständig/wettbewerblich.
		assert!(ROLLEN.contains(&"messstellenbetreiber_alt"));
		assert!(ROLLEN.contains(&"messstellenbetreiber_neu"));
		assert!(ROLLEN.contains(&"grundzustaendiger_messstellenbetreiber"));
		assert!(ROLLEN.contains(&"wettbewerblicher_messstellenbetreiber"));
	}

	#[test]
	fn rollen_no_duplicates() {
		let mut sorted = ROLLEN.to_vec();
		sorted.sort();
		sorted.dedup();
		assert_eq!(sorted.len(), ROLLEN.len(), "duplicate slug in ROLLEN");
	}
}

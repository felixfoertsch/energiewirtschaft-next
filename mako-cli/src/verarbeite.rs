use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::Local;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};

use crate::state_store::StateMap;

/// Load rollen.json from a markt directory, returning a map of MP-ID -> directory name.
fn load_rollen(markt: &Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
	let path = markt.join("rollen.json");
	let content = std::fs::read_to_string(&path)
		.map_err(|e| format!("rollen.json nicht gefunden in {}: {e}", markt.display()))?;
	let map: HashMap<String, String> = serde_json::from_str(&content)?;
	Ok(map)
}

/// Find the directory for a given MarktpartnerId within the markt directory.
fn find_rolle_dir(
	markt: &Path,
	mp_id: &MarktpartnerId,
	rollen: &HashMap<String, String>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
	let dir_name = rollen
		.get(mp_id.as_str())
		.ok_or_else(|| format!("Keine Rolle für MP-ID {} in rollen.json", mp_id.as_str()))?;
	Ok(markt.join(dir_name))
}

/// Write a CONTRL or APERAK quittung JSON file to the target directory's inbox.
fn write_quittung_file(
	rolle_dir: &Path,
	typ: &str,
	ergebnis: &mako_quittung::types::QuittungsErgebnis,
) -> Result<(), Box<dyn std::error::Error>> {
	let inbox = rolle_dir.join("inbox");
	std::fs::create_dir_all(&inbox)?;
	let filename = format!("{typ}.json");
	let content = serde_json::to_string_pretty(ergebnis)?;
	std::fs::write(inbox.join(filename), content)?;
	Ok(())
}

/// Short name for a payload type, used for outbox file naming.
fn payload_short_name(payload: &NachrichtenPayload) -> &'static str {
	match payload {
		NachrichtenPayload::UtilmdAnmeldung(_) => "utilmd_anmeldung",
		NachrichtenPayload::UtilmdBestaetigung(_) => "utilmd_bestaetigung",
		NachrichtenPayload::UtilmdAbmeldung(_) => "utilmd_abmeldung",
		NachrichtenPayload::UtilmdAblehnung(_) => "utilmd_ablehnung",
		NachrichtenPayload::UtilmdZuordnung(_) => "utilmd_zuordnung",
		NachrichtenPayload::UtilmdLieferendeAbmeldung(_) => "utilmd_lieferende_abmeldung",
		NachrichtenPayload::UtilmdLieferendeBestaetigung(_) => "utilmd_lieferende_bestaetigung",
		NachrichtenPayload::MsconsSchlussturnusmesswert(_) => "mscons_schlussturnusmesswert",
		NachrichtenPayload::UtilmdStammdatenaenderung(_) => "utilmd_stammdatenaenderung",
		NachrichtenPayload::UtilmdZuordnungsliste(_) => "utilmd_zuordnungsliste",
		NachrichtenPayload::UtilmdGeschaeftsdatenanfrage(_) => "utilmd_geschaeftsdatenanfrage",
		NachrichtenPayload::UtilmdGeschaeftsdatenantwort(_) => "utilmd_geschaeftsdatenantwort",
		_ => "nachricht",
	}
}

/// Map a Nachricht to an LfwEvent.
fn nachricht_to_lfw_event(
	nachricht: &Nachricht,
) -> Result<mako_gpke::v2025::lfw::LfwEvent, Box<dyn std::error::Error>> {
	use mako_gpke::v2025::lfw::LfwEvent;
	match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(a) => Ok(LfwEvent::AnmeldungEmpfangen(a.clone())),
		NachrichtenPayload::UtilmdAblehnung(a) => {
			Ok(LfwEvent::LfaHatAbgelehnt { grund: a.grund.clone() })
		}
		NachrichtenPayload::UtilmdBestaetigung(_) => {
			// Bestaetigung from NB to LFN = AnmeldungBestaetigt (LFA identity needed)
			// For simplicity, extract LFA from context or use a placeholder
			Ok(LfwEvent::LfaHatBestaetigt)
		}
		_ => Err(format!(
			"Kann {:?} nicht auf LfwEvent abbilden",
			std::mem::discriminant(&nachricht.payload)
		)
		.into()),
	}
}

/// Dispatch to the correct reducer based on process name.
fn dispatch_reducer(
	prozess: &str,
	key: &str,
	states: &StateMap,
	nachricht: &Nachricht,
) -> Result<(serde_json::Value, Vec<Nachricht>), Box<dyn std::error::Error>> {
	match prozess {
		"gpke_lfw" => dispatch_gpke_lfw(key, states, nachricht),
		other => Err(format!("Prozess '{other}' noch nicht implementiert").into()),
	}
}

/// Dispatch GPKE LFW events through the reducer.
fn dispatch_gpke_lfw(
	key: &str,
	states: &StateMap,
	nachricht: &Nachricht,
) -> Result<(serde_json::Value, Vec<Nachricht>), Box<dyn std::error::Error>> {
	use mako_gpke::v2025::lfw::{LfwState, reduce};

	// Deserialize state (default to Idle if not found)
	let state: LfwState = states
		.get(key)
		.map(|v| serde_json::from_value(v.clone()))
		.transpose()?
		.unwrap_or(LfwState::Idle);

	// Map Nachricht to LfwEvent
	let event = nachricht_to_lfw_event(nachricht)?;

	// Call reducer
	let output = reduce(state, event)?;

	// Serialize new state
	let new_state = serde_json::to_value(&output.state)?;
	Ok((new_state, output.nachrichten))
}

/// Update or create a .status.json file alongside the given datei path.
fn update_status(datei: &str, field: &str, value: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
	let status_path = format!("{}.status.json", datei);
	let mut status: serde_json::Map<String, serde_json::Value> =
		if std::path::Path::new(&status_path).exists() {
			serde_json::from_str(&std::fs::read_to_string(&status_path)?)?
		} else {
			serde_json::Map::new()
		};
	status.insert(field.to_string(), value);
	std::fs::write(&status_path, serde_json::to_string_pretty(&serde_json::Value::Object(status))?)?;
	Ok(())
}

/// Append a log entry to the markt log directory.
fn log_verarbeite_entry(
	markt: &Path,
	datei: &str,
	aktion: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	let log_dir = markt.join("log");
	std::fs::create_dir_all(&log_dir)?;
	let today = Local::now().format("%Y-%m-%d").to_string();
	let log_path = log_dir.join(format!("{today}.jsonl"));
	let entry = serde_json::json!({
		"zeitpunkt": Local::now().to_rfc3339(),
		"datei": datei,
		"aktion": aktion,
	});
	use std::io::Write;
	let mut file = std::fs::OpenOptions::new()
		.create(true)
		.append(true)
		.open(log_path)?;
	writeln!(file, "{}", serde_json::to_string(&entry)?)?;
	Ok(())
}

/// Process all unverarbeitet .edi and .json files in a role's inbox.
pub fn run_alle(markt: &str, rolle: &str) -> Result<(), Box<dyn std::error::Error>> {
	let inbox = std::path::Path::new(markt).join(rolle).join("inbox");
	let mut processed = 0;
	for entry in std::fs::read_dir(&inbox)? {
		let entry = entry?;
		let path = entry.path();
		let path_str = path.to_string_lossy().to_string();

		// Skip .status.json files
		if path_str.contains(".status.json") {
			continue;
		}

		// Only process .edi and .json files
		if !path.extension().map(|e| e == "edi" || e == "json").unwrap_or(false) {
			continue;
		}

		// Skip if already has "verarbeitet" status
		let status_path = inbox.join(format!("{}.status.json", path.file_name().unwrap().to_string_lossy()));
		if status_path.exists() {
			let content = std::fs::read_to_string(&status_path)?;
			if content.contains("verarbeitet") {
				continue;
			}
		}

		match run(path.to_str().unwrap(), markt) {
			Ok(()) => processed += 1,
			Err(e) => eprintln!("Fehler bei {}: {e}", path.display()),
		}
	}
	println!("{processed} Nachrichten verarbeitet in {rolle}/inbox/");
	Ok(())
}

pub fn run(datei: &str, markt: &str) -> Result<(), Box<dyn std::error::Error>> {
	// 1. Read file
	let content = std::fs::read_to_string(datei)
		.map_err(|e| format!("Datei '{datei}' nicht lesbar: {e}"))?;

	// 2. Parse (detect by extension)
	let nachricht: Nachricht = if datei.ends_with(".json") {
		serde_json::from_str(&content)?
	} else {
		mako_codec::edifact::dispatch::parse_nachricht(&content)?
	};

	// 3. Load rollen mapping and determine directories
	let markt_path = Path::new(markt);
	let rollen = load_rollen(markt_path)?;
	let empfaenger_dir = find_rolle_dir(markt_path, &nachricht.empfaenger, &rollen)?;
	let absender_dir = find_rolle_dir(markt_path, &nachricht.absender, &rollen)?;

	// 4. CONTRL check
	let contrl = mako_quittung::contrl::contrl_pruefen(&nachricht);
	write_quittung_file(&absender_dir, "contrl", &contrl)?;
	update_status(datei, "contrl", serde_json::to_value(&contrl)?)?;

	if matches!(contrl, mako_quittung::types::QuittungsErgebnis::Negativ(_)) {
		println!("CONTRL negativ — Verarbeitung gestoppt");
		log_verarbeite_entry(markt_path, datei, "contrl_negativ")?;
		return Ok(());
	}

	// 5. APERAK check
	let stichtag = chrono::Local::now().date_naive();
	let aperak = mako_quittung::aperak::aperak_pruefen(&nachricht, stichtag);
	write_quittung_file(&absender_dir, "aperak", &aperak)?;
	update_status(datei, "aperak", serde_json::to_value(&aperak)?)?;

	if matches!(aperak, mako_quittung::types::QuittungsErgebnis::Negativ(_)) {
		println!("APERAK negativ — Verarbeitung gestoppt");
		log_verarbeite_entry(markt_path, datei, "aperak_negativ")?;
		return Ok(());
	}

	// 6. Map to process
	let zuordnung = crate::event_mapping::zuordne_prozess(&nachricht)
		.ok_or("Kein Prozess für diesen Nachrichtentyp gefunden")?;

	// 7. Load state, dispatch reducer
	let mut states = crate::state_store::load_state(&empfaenger_dir);
	let (new_state, outgoing) =
		dispatch_reducer(&zuordnung.prozess, &zuordnung.key, &states, &nachricht)?;

	// 8. Save state
	states.insert(zuordnung.key.clone(), new_state);
	crate::state_store::save_state(&empfaenger_dir, &states);

	// 9. Write outgoing messages
	for msg in &outgoing {
		let filename = format!("{}.json", payload_short_name(&msg.payload));
		let json = serde_json::to_string_pretty(msg)?;
		let outbox = empfaenger_dir.join("outbox");
		std::fs::create_dir_all(&outbox)?;
		std::fs::write(outbox.join(&filename), &json)?;
		// Log each outgoing message
		let empfaenger_name = empfaenger_dir
			.file_name()
			.map(|n| n.to_string_lossy().to_string())
			.unwrap_or_default();
		log_verarbeite_entry(markt_path, &filename, &format!("gesendet_von_{empfaenger_name}"))?;
	}

	// 10. Update .status.json with verarbeitet timestamp
	update_status(datei, "verarbeitet", serde_json::Value::String(Local::now().to_rfc3339()))?;
	log_verarbeite_entry(markt_path, datei, "verarbeitet")?;

	println!(
		"Verarbeitet: {} — {} ausgehende Nachrichten",
		zuordnung.beschreibung,
		outgoing.len()
	);
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::NaiveDate;

	fn setup_markt() -> (tempfile::TempDir, PathBuf) {
		let tmp = tempfile::tempdir().expect("temp dir");
		let markt = tmp.path().join("markt");
		crate::init::run(markt.to_str().unwrap());
		(tmp, markt)
	}

	#[test]
	fn verarbeite_gpke_anmeldung() {
		let (_tmp, markt) = setup_markt();

		// Generate a UtilmdAnmeldung from lieferant_neu (index 0) to netzbetreiber (index 1)
		let lieferbeginn = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
		let nachricht = mako_testdata::utilmd::anmeldung(lieferbeginn);

		// The testdata uses mp_id(1) as LFN and mp_id(2) as NB.
		// Our init uses index 0 = lieferant_neu and index 1 = netzbetreiber.
		// We need to create a Nachricht that uses our init's MP-IDs.
		let lfn_id = MarktpartnerId::new(&crate::init::mp_id_for_index(0)).unwrap();
		let nb_id = MarktpartnerId::new(&crate::init::mp_id_for_index(1)).unwrap();

		let nachricht = Nachricht {
			absender: lfn_id.clone(),
			absender_rolle: mako_types::rolle::MarktRolle::LieferantNeu,
			empfaenger: nb_id.clone(),
			empfaenger_rolle: mako_types::rolle::MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(
				mako_types::gpke_nachrichten::UtilmdAnmeldung {
					malo_id: mako_testdata::ids::test_malo(0),
					lieferant_neu: lfn_id,
					lieferbeginn,
				},
			),
		};

		// Write Nachricht as JSON to netzbetreiber/inbox/
		let inbox = markt.join("netzbetreiber").join("inbox");
		let datei = inbox.join("anmeldung.json");
		let json = serde_json::to_string_pretty(&nachricht).unwrap();
		std::fs::write(&datei, &json).unwrap();

		// Run verarbeite
		let result = run(datei.to_str().unwrap(), markt.to_str().unwrap());
		assert!(result.is_ok(), "verarbeite fehlgeschlagen: {result:?}");

		// Check: CONTRL written to absender (lieferant_neu) inbox
		let contrl_path = markt
			.join("lieferant_neu")
			.join("inbox")
			.join("contrl.json");
		assert!(contrl_path.exists(), "CONTRL quittung nicht geschrieben");
		let contrl_content = std::fs::read_to_string(&contrl_path).unwrap();
		assert!(
			contrl_content.contains("Positiv"),
			"CONTRL sollte positiv sein"
		);

		// Check: APERAK written to absender (lieferant_neu) inbox
		let aperak_path = markt
			.join("lieferant_neu")
			.join("inbox")
			.join("aperak.json");
		assert!(aperak_path.exists(), "APERAK quittung nicht geschrieben");
		let aperak_content = std::fs::read_to_string(&aperak_path).unwrap();
		assert!(
			aperak_content.contains("Positiv"),
			"APERAK sollte positiv sein"
		);

		// Check: state.json updated for netzbetreiber
		let state_path = markt.join("netzbetreiber").join("state.json");
		let state_content = std::fs::read_to_string(&state_path).unwrap();
		assert!(
			state_content.contains("AnmeldungEingegangen"),
			"State sollte AnmeldungEingegangen sein, ist: {state_content}"
		);
	}

	#[test]
	fn verarbeite_aperak_negativ_stops_processing() {
		let (_tmp, markt) = setup_markt();

		// Anmeldung with lieferbeginn in the past -> APERAK negativ
		let lieferbeginn = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
		let lfn_id = MarktpartnerId::new(&crate::init::mp_id_for_index(0)).unwrap();
		let nb_id = MarktpartnerId::new(&crate::init::mp_id_for_index(1)).unwrap();

		let nachricht = Nachricht {
			absender: lfn_id.clone(),
			absender_rolle: mako_types::rolle::MarktRolle::LieferantNeu,
			empfaenger: nb_id,
			empfaenger_rolle: mako_types::rolle::MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(
				mako_types::gpke_nachrichten::UtilmdAnmeldung {
					malo_id: mako_testdata::ids::test_malo(0),
					lieferant_neu: lfn_id,
					lieferbeginn,
				},
			),
		};

		let inbox = markt.join("netzbetreiber").join("inbox");
		let datei = inbox.join("anmeldung_past.json");
		std::fs::write(&datei, serde_json::to_string_pretty(&nachricht).unwrap()).unwrap();

		let result = run(datei.to_str().unwrap(), markt.to_str().unwrap());
		assert!(result.is_ok());

		// APERAK should be negative
		let aperak_path = markt
			.join("lieferant_neu")
			.join("inbox")
			.join("aperak.json");
		let aperak_content = std::fs::read_to_string(&aperak_path).unwrap();
		assert!(
			aperak_content.contains("Negativ"),
			"APERAK sollte negativ sein"
		);

		// State should NOT be updated (still empty)
		let state_content =
			std::fs::read_to_string(markt.join("netzbetreiber").join("state.json")).unwrap();
		assert!(
			!state_content.contains("AnmeldungEingegangen"),
			"State sollte nicht aktualisiert worden sein"
		);
	}
}

use std::path::Path;
use chrono::Local;

pub fn run(markt: &str, von: &str, an: &str, datei: &str) -> Result<(), Box<dyn std::error::Error>> {
	let markt_path = Path::new(markt);
	let source = markt_path.join(von).join("outbox").join(datei);
	let dest_dir = markt_path.join(an).join("inbox");
	let dest = dest_dir.join(datei);

	if !source.exists() {
		return Err(format!("Datei nicht gefunden: {}", source.display()).into());
	}

	std::fs::create_dir_all(&dest_dir)?;
	std::fs::copy(&source, &dest)?;
	println!("Gesendet: {} → {}/inbox/{}", von, an, datei);

	// Write .status.json
	let status_path = dest_dir.join(format!("{}.status.json", datei));
	let status = serde_json::json!({
		"zugestellt": Local::now().to_rfc3339(),
	});
	std::fs::write(&status_path, serde_json::to_string_pretty(&status)?)?;

	// Append to log
	log_entry(markt_path, von, an, datei)?;

	Ok(())
}

pub fn log_entry(markt: &Path, von: &str, an: &str, datei: &str) -> Result<(), Box<dyn std::error::Error>> {
	let log_dir = markt.join("log");
	std::fs::create_dir_all(&log_dir)?;
	let today = Local::now().format("%Y-%m-%d").to_string();
	let log_path = log_dir.join(format!("{today}.jsonl"));
	let entry = serde_json::json!({
		"zeitpunkt": Local::now().to_rfc3339(),
		"von": von,
		"an": an,
		"datei": datei,
		"aktion": "gesendet",
	});
	use std::io::Write;
	let mut file = std::fs::OpenOptions::new()
		.create(true)
		.append(true)
		.open(log_path)?;
	writeln!(file, "{}", serde_json::to_string(&entry)?)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	fn setup_markt() -> (tempfile::TempDir, std::path::PathBuf) {
		let tmp = tempfile::tempdir().expect("temp dir");
		let markt = tmp.path().join("markt");
		crate::init::run(markt.to_str().unwrap());
		(tmp, markt)
	}

	#[test]
	fn test_sende_copies_file() {
		let (_tmp, markt) = setup_markt();
		let markt_str = markt.to_str().unwrap();

		// Place a file in lieferant_neu/outbox/
		let outbox = markt.join("lieferant_neu").join("outbox");
		std::fs::create_dir_all(&outbox).unwrap();
		let datei = "001_anmeldung.json";
		std::fs::write(outbox.join(datei), r#"{"test": true}"#).unwrap();

		// Sende from lieferant_neu to netzbetreiber
		let result = run(markt_str, "lieferant_neu", "netzbetreiber", datei);
		assert!(result.is_ok(), "sende fehlgeschlagen: {result:?}");

		// Verify file was copied to netzbetreiber/inbox/
		let dest = markt.join("netzbetreiber").join("inbox").join(datei);
		assert!(dest.exists(), "Datei nicht in inbox kopiert");
		let content = std::fs::read_to_string(&dest).unwrap();
		assert!(content.contains("\"test\": true"));

		// Verify .status.json was created with "zugestellt"
		let status_path = markt
			.join("netzbetreiber")
			.join("inbox")
			.join(format!("{}.status.json", datei));
		assert!(status_path.exists(), ".status.json nicht erstellt");
		let status_content = std::fs::read_to_string(&status_path).unwrap();
		assert!(
			status_content.contains("zugestellt"),
			".status.json sollte 'zugestellt' enthalten"
		);
	}

	#[test]
	fn test_sende_missing_file_returns_error() {
		let (_tmp, markt) = setup_markt();
		let result = run(markt.to_str().unwrap(), "lieferant_neu", "netzbetreiber", "does_not_exist.json");
		assert!(result.is_err(), "Fehlende Datei sollte Fehler erzeugen");
	}

	#[test]
	fn test_sende_writes_log_entry() {
		let (_tmp, markt) = setup_markt();
		let markt_str = markt.to_str().unwrap();

		let outbox = markt.join("lieferant_neu").join("outbox");
		std::fs::create_dir_all(&outbox).unwrap();
		let datei = "log_test.json";
		std::fs::write(outbox.join(datei), "{}").unwrap();

		run(markt_str, "lieferant_neu", "netzbetreiber", datei).unwrap();

		// Verify log file was written
		let today = chrono::Local::now().format("%Y-%m-%d").to_string();
		let log_path = markt.join("log").join(format!("{today}.jsonl"));
		assert!(log_path.exists(), "Log-Datei nicht erstellt");
		let log_content = std::fs::read_to_string(&log_path).unwrap();
		assert!(log_content.contains("gesendet"), "Log sollte 'gesendet' enthalten");
		assert!(log_content.contains("lieferant_neu"), "Log sollte Absender enthalten");
	}
}

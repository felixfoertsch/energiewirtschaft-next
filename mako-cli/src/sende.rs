use chrono::Local;
use std::path::Path;

pub struct Zustellung {
	pub zeitpunkt: String,
	pub quelle: std::path::PathBuf,
	pub ziel: std::path::PathBuf,
}

pub fn run(
	markt: &str,
	von: &str,
	an: &str,
	datei: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	let zustellung = zustellen(markt, von, an, datei)?;
	println!("Gesendet: {} → {}/inbox/{}", von, an, datei);
	println!("Status: {}", zustellung.zeitpunkt);
	Ok(())
}

pub fn zustellen(
	markt: &str,
	von: &str,
	an: &str,
	datei: &str,
) -> Result<Zustellung, Box<dyn std::error::Error>> {
	let markt_path = Path::new(markt);
	let source = markt_path.join(von).join("outbox").join(datei);
	let dest_dir = markt_path.join(an).join("inbox");
	let dest = dest_dir.join(datei);

	if !source.exists() {
		return Err(format!("Datei nicht gefunden: {}", source.display()).into());
	}

	std::fs::create_dir_all(&dest_dir)?;
	std::fs::copy(&source, &dest)?;

	let zeitpunkt = Local::now().to_rfc3339();
	update_status(
		&source,
		"zugestellt",
		serde_json::Value::String(zeitpunkt.clone()),
	)?;
	let status = read_status(&source)?;
	write_status(&dest, &status)?;

	log_entry(markt_path, von, an, datei)?;

	Ok(Zustellung {
		zeitpunkt,
		quelle: source,
		ziel: dest,
	})
}

pub fn update_status(
	datei: &Path,
	field: &str,
	value: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
	let mut status = read_status(datei)?;
	status.insert(field.to_string(), value);
	write_status(datei, &status)?;
	Ok(())
}

pub fn update_status_fields(
	datei: &Path,
	fields: &[(&str, serde_json::Value)],
) -> Result<(), Box<dyn std::error::Error>> {
	let mut status = read_status(datei)?;
	for (field, value) in fields {
		status.insert((*field).to_string(), value.clone());
	}
	write_status(datei, &status)?;
	Ok(())
}

fn read_status(
	datei: &Path,
) -> Result<serde_json::Map<String, serde_json::Value>, Box<dyn std::error::Error>> {
	let path = status_path(datei);
	if !path.exists() {
		return Ok(serde_json::Map::new());
	}
	let content = std::fs::read_to_string(&path)?;
	let status = serde_json::from_str(&content)?;
	Ok(status)
}

fn write_status(
	datei: &Path,
	status: &serde_json::Map<String, serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
	let path = status_path(datei);
	std::fs::write(
		&path,
		serde_json::to_string_pretty(&serde_json::Value::Object(status.clone()))?,
	)?;
	Ok(())
}

fn status_path(datei: &Path) -> std::path::PathBuf {
	let file_name = datei
		.file_name()
		.map(|name| name.to_string_lossy().to_string())
		.unwrap_or_else(|| datei.to_string_lossy().to_string());
	datei.with_file_name(format!("{file_name}.status.json"))
}

pub fn log_entry(
	markt: &Path,
	von: &str,
	an: &str,
	datei: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
		let result = run(
			markt.to_str().unwrap(),
			"lieferant_neu",
			"netzbetreiber",
			"does_not_exist.json",
		);
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
		assert!(
			log_content.contains("gesendet"),
			"Log sollte 'gesendet' enthalten"
		);
		assert!(
			log_content.contains("lieferant_neu"),
			"Log sollte Absender enthalten"
		);
	}
}

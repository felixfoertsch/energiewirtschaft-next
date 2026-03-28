use std::path::Path;

use mako_verify::referenzdaten::Referenzdaten;

pub fn run(verzeichnis: &str, referenzdaten_pfad: &str) -> Result<(), Box<dyn std::error::Error>> {
	let verzeichnis_pfad = Path::new(verzeichnis);
	if !verzeichnis_pfad.is_dir() {
		return Err(format!("Verzeichnis '{verzeichnis}' nicht gefunden").into());
	}

	let refdata = Referenzdaten::laden(Path::new(referenzdaten_pfad), "FV2504", "FV2604");
	let ergebnis = mako_verify::verifiziere_batch(verzeichnis_pfad, &refdata);

	println!("{}", ergebnis.zusammenfassung());

	let report_pfad = verzeichnis_pfad.join("verifikation.json");
	let json = serde_json::to_string_pretty(&ergebnis)?;
	std::fs::write(&report_pfad, &json)?;
	println!("Bericht geschrieben: {}", report_pfad.display());

	Ok(())
}

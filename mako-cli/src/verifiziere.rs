use std::path::Path;

use mako_verify::referenzdaten::Referenzdaten;

pub fn run(datei: &str, referenzdaten_pfad: &str) -> Result<(), Box<dyn std::error::Error>> {
	let inhalt = std::fs::read_to_string(datei)
		.map_err(|e| format!("Datei '{datei}' nicht lesbar: {e}"))?;

	let refdata = Referenzdaten::laden(Path::new(referenzdaten_pfad), "FV2504", "FV2604");
	let ergebnis = mako_verify::verifiziere_nachricht(&inhalt, &refdata);

	let json = serde_json::to_string_pretty(&ergebnis)?;
	println!("{json}");

	Ok(())
}

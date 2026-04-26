use std::io::Write;

use mako_types::nachricht::schema_for;

pub fn run(typ: &str) -> Result<(), Box<dyn std::error::Error>> {
	let stdout = std::io::stdout();
	let mut lock = stdout.lock();
	run_to_writer(typ, &mut lock)
}

pub fn run_to_writer(
	typ: &str,
	writer: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
	let schema = schema_for(typ).ok_or_else(|| format!("Unbekannter Payload-Typ: {typ}"))?;
	serde_json::to_writer_pretty(&mut *writer, &schema)?;
	writeln!(writer)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn schema_json_utilmd_anmeldung_is_valid_json() {
		let mut out = Vec::new();
		run_to_writer("UtilmdAnmeldung", &mut out).unwrap();
		let json: serde_json::Value = serde_json::from_slice(&out).unwrap();
		assert_eq!(json.get("title").and_then(|v| v.as_str()), Some("UtilmdAnmeldung"));
		assert!(json.get("properties").and_then(|v| v.as_object()).is_some());
	}
}

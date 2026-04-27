//! Aggregator: collects per-crate process catalogs into a single Vec.
//!
//! Add a new process crate by importing it here and chaining its
//! `katalog()` into [`alle_prozesse`]. Tests in this crate enforce that
//! every market role from `mako-cli`'s init catalog is reachable through
//! at least one process — that prevents a future role from being added
//! to the role list without a corresponding process surface.

use mako_types::katalog::ProzessDef;

pub fn alle_prozesse() -> Vec<ProzessDef> {
	let mut out: Vec<ProzessDef> = Vec::new();
	out.extend(mako_gpke::v2025::katalog::katalog());
	out.extend(mako_geli::v2025::katalog::katalog());
	out.extend(mako_wim::v2025::katalog::katalog());
	out.extend(mako_mabis::v2025::katalog::katalog());
	out.extend(mako_gabi::v2025::katalog::katalog());
	out.extend(mako_ubp::v2025::katalog::katalog());
	out.extend(mako_abrechnung::v2025::katalog::katalog());
	out.extend(mako_kov::v2025::katalog::katalog());
	out.extend(mako_rd2::v2025::katalog::katalog());
	out.extend(mako_mpes::v2025::katalog::katalog());
	out.extend(mako_14a::v2025::katalog::katalog());
	out
}

/// Render the full catalog as JSON for consumption by the test UI.
pub fn katalog_als_json() -> String {
	serde_json::to_string_pretty(&alle_prozesse()).expect("ProzessDef is serializable")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn katalog_ist_nicht_leer() {
		let prozesse = alle_prozesse();
		assert!(prozesse.len() >= 30, "expected 30+ Prozesse, got {}", prozesse.len());
	}

	#[test]
	fn keys_sind_eindeutig() {
		let prozesse = alle_prozesse();
		let mut keys: Vec<_> = prozesse.iter().map(|p| p.key.clone()).collect();
		keys.sort();
		let urspruenglich = keys.len();
		keys.dedup();
		assert_eq!(keys.len(), urspruenglich, "duplicate process key in katalog");
	}

	#[test]
	fn json_serialisiert_ohne_fehler() {
		let json = katalog_als_json();
		assert!(json.starts_with("["));
		assert!(json.contains("\"key\""));
		assert!(json.contains("\"schritte\""));
		assert!(json.contains("\"erklaerung\""));
	}

	#[test]
	fn jeder_schritt_hat_eine_erklaerung() {
		let prozesse = alle_prozesse();
		let fehlende: Vec<_> = prozesse
			.iter()
			.flat_map(|p| p.schritte.iter().map(move |s| (&p.key, &s.name, &s.erklaerung)))
			.filter(|(_, _, erklaerung)| erklaerung.trim().is_empty())
			.collect();
		assert!(fehlende.is_empty(), "Schritte ohne Erklärung: {fehlende:?}");
	}
}

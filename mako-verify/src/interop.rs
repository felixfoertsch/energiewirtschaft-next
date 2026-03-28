use std::collections::HashMap;

use mako_types::nachricht::{Nachricht, NachrichtenPayload};

use crate::bericht::{InteropErgebnis, InteropFeldVergleich, Urteil};

/// Extract key fields from a parsed `Nachricht` into a flat string map.
/// This map is the basis for cross-validation against a third-party parser.
pub fn extrahiere_schluesselfelder(nachricht: &Nachricht) -> HashMap<String, String> {
	let mut felder = HashMap::new();

	felder.insert("absender".into(), nachricht.absender.as_str().to_string());
	felder.insert("empfaenger".into(), nachricht.empfaenger.as_str().to_string());
	felder.insert("absender_rolle".into(), format!("{:?}", nachricht.absender_rolle));
	felder.insert("empfaenger_rolle".into(), format!("{:?}", nachricht.empfaenger_rolle));

	if let Some(ref pid) = nachricht.pruef_id {
		felder.insert("pruefidentifikator".into(), pid.code().to_string());
	}

	// payload-specific fields
	match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(anm) => {
			felder.insert("malo_id".into(), anm.malo_id.as_str().to_string());
			felder.insert("lieferant_neu".into(), anm.lieferant_neu.as_str().to_string());
			felder.insert("lieferbeginn".into(), anm.lieferbeginn.to_string());
		}
		NachrichtenPayload::UtilmdBestaetigung(best) => {
			felder.insert("malo_id".into(), best.malo_id.as_str().to_string());
			felder.insert("bestaetigt_fuer".into(), best.bestaetigt_fuer.as_str().to_string());
			felder.insert("lieferbeginn".into(), best.lieferbeginn.to_string());
		}
		NachrichtenPayload::UtilmdAbmeldung(abm) => {
			felder.insert("malo_id".into(), abm.malo_id.as_str().to_string());
			felder.insert("lieferende".into(), abm.lieferende.to_string());
		}
		NachrichtenPayload::UtilmdAblehnung(abl) => {
			felder.insert("malo_id".into(), abl.malo_id.as_str().to_string());
			felder.insert("grund".into(), format!("{:?}", abl.grund));
		}
		// further payload variants can be added as needed
		_ => {}
	}

	felder
}

/// Compare two field maps (ours vs. third-party) and produce an `InteropErgebnis`.
/// Only fields present in both maps are compared.
pub fn vergleiche_felder(
	unsere: &HashMap<String, String>,
	drittanbieter: &HashMap<String, String>,
) -> InteropErgebnis {
	let mut feldvergleiche = Vec::new();

	for (key, unser_wert) in unsere {
		if let Some(dritt_wert) = drittanbieter.get(key) {
			feldvergleiche.push(InteropFeldVergleich {
				feld: key.clone(),
				unser_wert: Some(unser_wert.clone()),
				drittanbieter_wert: Some(dritt_wert.clone()),
				stimmt_ueberein: unser_wert == dritt_wert,
			});
		}
	}

	// sort for deterministic output
	feldvergleiche.sort_by(|a, b| a.feld.cmp(&b.feld));

	let alle_stimmen = feldvergleiche.iter().all(|f| f.stimmt_ueberein);
	let urteil = if feldvergleiche.is_empty() {
		Urteil::NichtPruefbar
	} else if alle_stimmen {
		Urteil::Bestanden
	} else {
		Urteil::Fehlgeschlagen
	};

	InteropErgebnis {
		parse_ok_unser: true,
		parse_ok_drittanbieter: true,
		roundtrip_ok: alle_stimmen,
		feldvergleiche,
		urteil,
	}
}

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use chrono::Local;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ErstelleNachrichtInput {
	empfaenger_slug: String,
	empfaenger_id: String,
	typ: Option<String>,
	fields: serde_json::Value,
	#[serde(default)]
	auto_zustellen: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum WireFormat {
	Edifact,
	Xml,
	Binary,
}

#[derive(Debug, Serialize)]
struct ErstelleNachrichtOutput {
	ok: bool,
	wire_format: WireFormat,
	#[serde(skip_serializing_if = "Option::is_none")]
	datei: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	validierung: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	fehler: Option<String>,
}

struct CommandError {
	wire_format: WireFormat,
	message: String,
}

struct Success {
	wire_format: WireFormat,
	datei: PathBuf,
	validierung: Option<serde_json::Value>,
}

pub fn run(rolle: &str, markt: &str) -> Result<(), Box<dyn std::error::Error>> {
	run_with_options(rolle, markt, true, None)
}

pub fn run_with_auto_zustellen(
	rolle: &str,
	markt: &str,
	auto_zustellen: bool,
) -> Result<(), Box<dyn std::error::Error>> {
	run_with_options(rolle, markt, auto_zustellen, None)
}

pub fn run_with_options(
	rolle: &str,
	markt: &str,
	auto_zustellen: bool,
	typ_override: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
	let stdin = std::io::stdin();
	let stdout = std::io::stdout();
	let mut input = stdin.lock();
	let mut output = stdout.lock();
	run_with_reader_writer(
		rolle,
		markt,
		auto_zustellen,
		typ_override,
		&mut input,
		&mut output,
	)
}

pub fn run_with_reader_writer(
	rolle: &str,
	markt: &str,
	auto_zustellen: bool,
	typ_override: Option<&str>,
	reader: &mut impl Read,
	writer: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
	let output = match execute(rolle, markt, auto_zustellen, typ_override, reader) {
		Ok(success) => ErstelleNachrichtOutput {
			ok: true,
			wire_format: success.wire_format,
			datei: Some(success.datei.display().to_string()),
			validierung: success.validierung,
			fehler: None,
		},
		Err(error) => ErstelleNachrichtOutput {
			ok: false,
			wire_format: error.wire_format,
			datei: None,
			validierung: None,
			fehler: Some(error.message),
		},
	};

	serde_json::to_writer_pretty(&mut *writer, &output)?;
	writeln!(writer)?;
	Ok(())
}

fn execute(
	rolle: &str,
	markt: &str,
	auto_zustellen: bool,
	typ_override: Option<&str>,
	reader: &mut impl Read,
) -> Result<Success, CommandError> {
	let mut raw = String::new();
	reader.read_to_string(&mut raw).map_err(|e| CommandError {
		wire_format: WireFormat::Edifact,
		message: format!("stdin konnte nicht gelesen werden: {e}"),
	})?;

	let input: ErstelleNachrichtInput = serde_json::from_str(&raw).map_err(|e| CommandError {
		wire_format: WireFormat::Edifact,
		message: format!("stdin ist kein gültiges JSON: {e}"),
	})?;
	let typ = typ_override
		.map(str::to_string)
		.or(input.typ)
		.ok_or_else(|| CommandError {
			wire_format: WireFormat::Edifact,
			message: "Payload-Typ fehlt; setze stdin.typ oder --typ".to_string(),
		})?;
	let wire_format = wire_format_for_typ(&typ);
	let auto_zustellen = input.auto_zustellen.unwrap_or(auto_zustellen);

	let payload = NachrichtenPayload::from_value_for_typ(&typ, input.fields)
		.map_err(|e| CommandError {
			wire_format,
			message: format!("Payload-Felder für {typ} sind ungültig: {e}"),
		})?
		.ok_or_else(|| CommandError {
			wire_format,
			message: format!("Unbekannter Payload-Typ: {typ}"),
		})?;

	let markt_path = Path::new(markt);
	let rollen = load_rollen(markt_path).map_err(|e| CommandError {
		wire_format,
		message: e,
	})?;
	let absender = absender_id_for_slug(&rollen, rolle).map_err(|e| CommandError {
		wire_format,
		message: e,
	})?;
	let absender_rolle = MarktRolle::from_slug(rolle).ok_or_else(|| CommandError {
		wire_format,
		message: format!("Unbekannte Absenderrolle: {rolle}"),
	})?;
	let empfaenger = MarktpartnerId::new(&input.empfaenger_id).map_err(|e| CommandError {
		wire_format,
		message: format!("empfaenger_id ist ungültig: {e}"),
	})?;
	let empfaenger_slug = slug_for_id(&rollen, &empfaenger).map_err(|e| CommandError {
		wire_format,
		message: e,
	})?;
	if empfaenger_slug != input.empfaenger_slug {
		return Err(CommandError {
			wire_format,
			message: format!(
				"empfaenger_slug '{}' passt nicht zu empfaenger_id '{}'; erwartet '{}'",
				input.empfaenger_slug, input.empfaenger_id, empfaenger_slug
			),
		});
	}
	let empfaenger_rolle = MarktRolle::from_slug(&empfaenger_slug).ok_or_else(|| CommandError {
		wire_format,
		message: format!("Unbekannte Empfängerrolle: {empfaenger_slug}"),
	})?;

	let nachricht = Nachricht {
		absender,
		absender_rolle,
		empfaenger,
		empfaenger_rolle,
		pruef_id: PruefIdentifikator::for_payload(&payload),
		payload,
	};

	let bytes = serialize_wire(&nachricht, wire_format).map_err(|e| CommandError {
		wire_format,
		message: e,
	})?;
	let datei =
		write_outbox_file(markt_path, rolle, &typ, wire_format, &bytes).map_err(|e| {
			CommandError {
				wire_format,
				message: e,
			}
		})?;
	let erstellt = Local::now().to_rfc3339();
	crate::sende::update_status(&datei, "erstellt", serde_json::Value::String(erstellt)).map_err(
		|e| CommandError {
			wire_format,
			message: format!("Status konnte nicht geschrieben werden: {e}"),
		},
	)?;

	if auto_zustellen {
		auto_zustellen_mit_ack(markt_path, rolle, &empfaenger_slug, &datei, &nachricht).map_err(
			|e| CommandError {
				wire_format,
				message: e,
			},
		)?;
	}

	let validierung = if wire_format == WireFormat::Edifact {
		let edifact = String::from_utf8(bytes).map_err(|e| CommandError {
			wire_format,
			message: format!("EDIFACT ist kein UTF-8: {e}"),
		})?;
		Some(verify_edifact(&edifact).map_err(|e| CommandError {
			wire_format,
			message: e,
		})?)
	} else {
		None
	};

	Ok(Success {
		wire_format,
		datei,
		validierung,
	})
}

fn load_rollen(markt: &Path) -> Result<HashMap<String, String>, String> {
	let path = markt.join("rollen.json");
	let content = std::fs::read_to_string(&path)
		.map_err(|e| format!("rollen.json nicht gefunden in {}: {e}", markt.display()))?;
	serde_json::from_str(&content).map_err(|e| {
		format!(
			"rollen.json ist kein gültiges JSON ({}): {e}",
			path.display()
		)
	})
}

fn absender_id_for_slug(
	rollen: &HashMap<String, String>,
	rolle: &str,
) -> Result<MarktpartnerId, String> {
	let Some((mp_id, _)) = rollen.iter().find(|(_, slug)| slug.as_str() == rolle) else {
		return Err(format!("Keine MP-ID für Rolle '{rolle}' in rollen.json"));
	};
	MarktpartnerId::new(mp_id).map_err(|e| format!("MP-ID für Rolle '{rolle}' ist ungültig: {e}"))
}

fn slug_for_id(rollen: &HashMap<String, String>, mp_id: &MarktpartnerId) -> Result<String, String> {
	rollen
		.get(mp_id.as_str())
		.cloned()
		.ok_or_else(|| format!("Keine Rolle für MP-ID {} in rollen.json", mp_id.as_str()))
}

fn wire_format_for_typ(typ: &str) -> WireFormat {
	if typ == "ClsSteuersignal" {
		WireFormat::Binary
	} else if typ.starts_with("Rd") || typ == "AcknowledgementDocument" {
		WireFormat::Xml
	} else {
		WireFormat::Edifact
	}
}

fn auto_zustellen_mit_ack(
	markt: &Path,
	absender_slug: &str,
	empfaenger_slug: &str,
	original_datei: &Path,
	nachricht: &Nachricht,
) -> Result<(), String> {
	let dateiname = original_datei
		.file_name()
		.map(|name| name.to_string_lossy().to_string())
		.ok_or_else(|| format!("Dateiname fehlt: {}", original_datei.display()))?;

	crate::sende::zustellen(
		&markt.to_string_lossy(),
		absender_slug,
		empfaenger_slug,
		&dateiname,
	)
	.map_err(|e| format!("Auto-Zustellung fehlgeschlagen: {e}"))?;

	if matches!(
		nachricht.payload,
		NachrichtenPayload::AcknowledgementDocument(_)
	) {
		return Ok(());
	}

	let ack = mako_quittung::acknowledgement::erzeuge_ack(nachricht);
	let ack_bytes = serialize_wire(&ack, WireFormat::Xml)?;
	let ack_datei = write_outbox_file(
		markt,
		empfaenger_slug,
		"AcknowledgementDocument",
		WireFormat::Xml,
		&ack_bytes,
	)?;
	crate::sende::update_status(
		&ack_datei,
		"erstellt",
		serde_json::Value::String(Local::now().to_rfc3339()),
	)
	.map_err(|e| format!("ACK-Status konnte nicht geschrieben werden: {e}"))?;

	let ack_dateiname = ack_datei
		.file_name()
		.map(|name| name.to_string_lossy().to_string())
		.ok_or_else(|| format!("ACK-Dateiname fehlt: {}", ack_datei.display()))?;
	let zustellung = crate::sende::zustellen(
		&markt.to_string_lossy(),
		empfaenger_slug,
		absender_slug,
		&ack_dateiname,
	)
	.map_err(|e| format!("ACK-Zustellung fehlgeschlagen: {e}"))?;

	crate::sende::update_status_fields(
		original_datei,
		&[
			(
				"ack_zugestellt",
				serde_json::Value::String(zustellung.zeitpunkt.clone()),
			),
			(
				"ack",
				serde_json::json!({
					"ergebnis": "positiv",
					"zeitpunkt": zustellung.zeitpunkt,
				}),
			),
		],
	)
	.map_err(|e| format!("ACK-Status konnte nicht aktualisiert werden: {e}"))?;

	Ok(())
}

fn serialize_wire(nachricht: &Nachricht, wire_format: WireFormat) -> Result<Vec<u8>, String> {
	match wire_format {
		WireFormat::Edifact => mako_codec::edifact::dispatch::serialize_nachricht(nachricht)
			.map(|s| s.into_bytes())
			.map_err(|e| format!("EDIFACT-Serialisierung fehlgeschlagen: {e}")),
		WireFormat::Xml => mako_codec::xml::serializer::serialize_xml(nachricht)
			.map(|s| s.into_bytes())
			.map_err(|e| format!("XML-Serialisierung fehlgeschlagen: {e}")),
		WireFormat::Binary => serde_json::to_vec(nachricht)
			.map_err(|e| format!("CLS-Binary-Serialisierung fehlgeschlagen: {e}")),
	}
}

fn write_outbox_file(
	markt: &Path,
	rolle: &str,
	typ: &str,
	wire_format: WireFormat,
	bytes: &[u8],
) -> Result<PathBuf, String> {
	let outbox = markt.join(rolle).join("outbox");
	std::fs::create_dir_all(&outbox).map_err(|e| {
		format!(
			"outbox konnte nicht erstellt werden ({}): {e}",
			outbox.display()
		)
	})?;
	let existing = std::fs::read_dir(&outbox)
		.map_err(|e| {
			format!(
				"outbox konnte nicht gelesen werden ({}): {e}",
				outbox.display()
			)
		})?
		.filter_map(|entry| entry.ok())
		.filter(|entry| {
			entry.path().is_file()
				&& !entry
					.file_name()
					.to_string_lossy()
					.ends_with(".status.json")
		})
		.count();
	let extension = match wire_format {
		WireFormat::Edifact => "edi",
		WireFormat::Xml => "xml",
		WireFormat::Binary => "bin",
	};
	let filename = format!("{:03}_{}.{}", existing + 1, typ, extension);
	let datei = outbox.join(filename);
	std::fs::write(&datei, bytes).map_err(|e| {
		format!(
			"Nachricht konnte nicht geschrieben werden ({}): {e}",
			datei.display()
		)
	})?;
	Ok(datei)
}

fn verify_edifact(edifact: &str) -> Result<serde_json::Value, String> {
	let ref_path = [
		"mako-verify/referenzdaten",
		"../mako-verify/referenzdaten",
		"referenzdaten",
	]
	.iter()
	.map(Path::new)
	.find(|path| path.exists())
	.unwrap_or_else(|| Path::new("referenzdaten"));
	let refdata = mako_verify::referenzdaten::Referenzdaten::laden(ref_path, "FV2504", "FV2604");
	let ergebnis = mako_verify::verifiziere_nachricht(edifact, &refdata);
	serde_json::to_value(ergebnis)
		.map_err(|e| format!("Validierungsergebnis konnte nicht serialisiert werden: {e}"))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn erstelle_nachricht_utilmd_anmeldung_writes_edi() {
		let tmp = tempfile::tempdir().unwrap();
		let markt = tmp.path().join("markt");
		crate::init::run(markt.to_str().unwrap());

		let input = br#"{
			"empfaenger_slug": "netzbetreiber",
			"empfaenger_id": "9900000000001",
			"typ": "UtilmdAnmeldung",
			"fields": {
				"malo_id": "51238696788",
				"lieferant_neu": "9900000000003",
			"lieferbeginn": "2026-07-01"
			}
		}"#;

		let mut reader = &input[..];
		let mut out = Vec::new();
		run_with_reader_writer(
			"lieferant_neu",
			markt.to_str().unwrap(),
			true,
			None,
			&mut reader,
			&mut out,
		)
		.unwrap();

		let json: serde_json::Value = serde_json::from_slice(&out).unwrap();
		assert_eq!(json.get("ok").and_then(|v| v.as_bool()), Some(true));
		assert_eq!(
			json.get("wire_format").and_then(|v| v.as_str()),
			Some("edifact")
		);
		let datei = json.get("datei").and_then(|v| v.as_str()).unwrap();
		assert!(datei.ends_with(".edi"), "expected .edi file, got {datei}");
		assert!(Path::new(datei).exists(), "expected output file to exist");

		let dateiname = Path::new(datei)
			.file_name()
			.and_then(|name| name.to_str())
			.unwrap();
		assert!(
			markt
				.join("netzbetreiber")
				.join("inbox")
				.join(dateiname)
				.exists(),
			"Nachricht sollte automatisch in der Empfänger-Inbox liegen"
		);
		assert!(
			markt
				.join("lieferant_neu")
				.join("inbox")
				.join("001_AcknowledgementDocument.xml")
				.exists(),
			"ACK sollte automatisch in der Sender-Inbox liegen"
		);
		let status_path = Path::new(datei).with_file_name(format!("{dateiname}.status.json"));
		let status: serde_json::Value =
			serde_json::from_str(&std::fs::read_to_string(status_path).unwrap()).unwrap();
		assert!(
			status.get("zugestellt").is_some(),
			"Status braucht zugestellt"
		);
		assert!(
			status.get("ack_zugestellt").is_some(),
			"Status braucht ack_zugestellt"
		);
		assert_eq!(
			status
				.get("ack")
				.and_then(|ack| ack.get("ergebnis"))
				.and_then(|v| v.as_str()),
			Some("positiv")
		);
	}
}

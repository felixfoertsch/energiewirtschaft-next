//! Pure functions that build individual EDIFACT segment strings.
//! These produce raw segment text — parsing into typed `Segment` structs is dispatch.rs territory.

use chrono::{Datelike, NaiveDate};

pub fn unb(sender: &str, empfaenger: &str, referenz: &str) -> String {
	format!("UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+{referenz}'")
}

pub fn unh(referenz: &str, typ: &str, version: &str) -> String {
	format!("UNH+{referenz}+{typ}:{version}'")
}

pub fn bgm(qualifier: &str, dok_nr: &str) -> String {
	format!("BGM+{qualifier}+{dok_nr}'")
}

/// BGM with additional fields (e.g. INVOIC: BGM+380+RG-2026-0001)
pub fn bgm_full(qualifier: &str, dok_nr: &str, extra: &str) -> String {
	if extra.is_empty() {
		bgm(qualifier, dok_nr)
	} else {
		format!("BGM+{qualifier}+{dok_nr}+{extra}'")
	}
}

pub fn dtm_102(qualifier: &str, date: NaiveDate) -> String {
	format!(
		"DTM+{qualifier}:{:04}{:02}{:02}:102'",
		date.year(),
		date.month(),
		date.day()
	)
}

pub fn dtm_203(qualifier: &str, datetime: &str) -> String {
	format!("DTM+{qualifier}:{datetime}:203'")
}

/// DTM+137 with timezone offset (the standard creation timestamp format)
pub fn dtm_137() -> String {
	"DTM+137:20260325120000?+01:303'".to_string()
}

pub fn nad(qualifier: &str, mp_id: &str) -> String {
	format!("NAD+{qualifier}+{mp_id}::293'")
}

pub fn nad_with_contact(qualifier: &str, mp_id: &str) -> String {
	format!("NAD+{qualifier}+{mp_id}::293'")
}

pub fn cta(qualifier: &str, name: &str) -> String {
	format!("CTA+{qualifier}+:{name}'")
}

pub fn rff(qualifier: &str, value: &str) -> String {
	format!("RFF+{qualifier}:{value}'")
}

pub fn rff_z13(pid_code: u32) -> String {
	format!("RFF+Z13:{pid_code}'")
}

pub fn ide(qualifier: &str, id: &str) -> String {
	format!("IDE+{qualifier}+{id}'")
}

pub fn ide_24(id: &str) -> String {
	ide("24", id)
}

pub fn loc(qualifier: &str, id: &str) -> String {
	format!("LOC+{qualifier}+{id}'")
}

pub fn sts(qualifier: &str, code: &str) -> String {
	format!("STS+{qualifier}++{code}'")
}

/// STS without qualifier prefix — bare `STS+CODE'`
pub fn sts_bare(code: &str) -> String {
	format!("STS+{code}'")
}

pub fn ftx(qualifier: &str, text: &str) -> String {
	format!("FTX+{qualifier}++{text}'")
}

pub fn qty(qualifier: &str, value: &str, unit: &str) -> String {
	format!("QTY+{qualifier}:{value}:{unit}'")
}

pub fn moa(qualifier: &str, value: &str) -> String {
	format!("MOA+{qualifier}:{value}'")
}

pub fn imd(text: &str) -> String {
	format!("IMD+F++:::{text}'")
}

pub fn lin(pos: u32, bezeichnung: &str) -> String {
	format!("LIN+{pos}++{bezeichnung}'")
}

pub fn pri(qualifier: &str, value: &str) -> String {
	format!("PRI+{qualifier}:{value}'")
}

pub fn mea(qualifier1: &str, qualifier2: &str, unit: &str) -> String {
	format!("MEA+{qualifier1}+{qualifier2}+{unit}'")
}

pub fn cci(value: &str) -> String {
	format!("CCI+{value}'")
}

pub fn cci_full(qualifier: &str, sub1: &str, sub2: &str) -> String {
	format!("CCI+{qualifier}:{sub1}:{sub2}'")
}

pub fn cci_with_value(qualifier: &str, empty: &str, value: &str) -> String {
	format!("CCI+{qualifier}+{empty}+{value}'")
}

pub fn cci_z30_typed(_typ: &str, value: &str) -> String {
	format!("CCI+Z30::{value}'")
}

pub fn cav(value: &str) -> String {
	format!("CAV+{value}'")
}

pub fn unt(count: u32, referenz: &str) -> String {
	format!("UNT+{count}+{referenz}'")
}

pub fn unz(count: u32, referenz: &str) -> String {
	format!("UNZ+{count}+{referenz}'")
}

/// Wrap body segments into a complete EDIFACT interchange.
/// Adds UNB, UNH, UNT, UNZ around body. DTM+137 and NAD segments are part of body.
pub fn nachricht(sender: &str, empfaenger: &str, typ: &str, version: &str, body: Vec<String>) -> String {
	let ref_nr = "00001";
	// UNT counts: UNH + body segments + UNT itself (= body.len() + 2)
	let unt_count = body.len() as u32 + 2;
	let mut parts = vec![
		unb(sender, empfaenger, ref_nr),
		unh("1", typ, version),
	];
	parts.extend(body);
	parts.push(unt(unt_count, "1"));
	parts.push(unz(1, ref_nr));
	parts.join("")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unb_format() {
		let s = unb("9900000000000", "9900000000001", "00001");
		assert!(s.starts_with("UNB+"));
		assert!(s.ends_with('\''));
		assert!(s.contains("9900000000000:500"));
	}

	#[test]
	fn unh_format() {
		let s = unh("1", "UTILMD", "D:11A:UN:S2.1");
		assert_eq!(s, "UNH+1+UTILMD:D:11A:UN:S2.1'");
	}

	#[test]
	fn bgm_format() {
		let s = bgm("E01", "DOK00001");
		assert_eq!(s, "BGM+E01+DOK00001'");
	}

	#[test]
	fn dtm_102_format() {
		let d = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();
		let s = dtm_102("92", d);
		assert_eq!(s, "DTM+92:20260701:102'");
	}

	#[test]
	fn dtm_137_format() {
		let s = dtm_137();
		assert_eq!(s, "DTM+137:20260325120000?+01:303'");
	}

	#[test]
	fn nad_format() {
		let s = nad("MS", "9900000000000");
		assert_eq!(s, "NAD+MS+9900000000000::293'");
	}

	#[test]
	fn rff_z13_format() {
		let s = rff_z13(44001);
		assert_eq!(s, "RFF+Z13:44001'");
	}

	#[test]
	fn ide_24_format() {
		let s = ide_24("51238696700");
		assert_eq!(s, "IDE+24+51238696700'");
	}

	#[test]
	fn sts_format() {
		let s = sts("7", "Z08");
		assert_eq!(s, "STS+7++Z08'");
	}

	#[test]
	fn qty_format() {
		let s = qty("220", "12345.6", "kWh");
		assert_eq!(s, "QTY+220:12345.6:kWh'");
	}

	#[test]
	fn moa_format() {
		let s = moa("203", "12500");
		assert_eq!(s, "MOA+203:12500'");
	}

	#[test]
	fn unt_format() {
		let s = unt(9, "1");
		assert_eq!(s, "UNT+9+1'");
	}

	#[test]
	fn unz_format() {
		let s = unz(1, "00001");
		assert_eq!(s, "UNZ+1+00001'");
	}

	#[test]
	fn nachricht_wraps_body() {
		let result = nachricht(
			"9900000000000",
			"9900000000001",
			"UTILMD",
			"D:11A:UN:S2.1",
			vec![bgm("E01", "DOK00001")],
		);
		assert!(result.starts_with("UNB+"));
		assert!(result.contains("UNH+1+UTILMD:D:11A:UN:S2.1'"));
		assert!(result.contains("BGM+E01+DOK00001'"));
		// UNT count: UNH + 1 body + UNT = 3
		assert!(result.contains("UNT+3+1'"));
		assert!(result.ends_with("UNZ+1+00001'"));
	}
}

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use super::parser::ParseError;
use super::segment::{Element, Segment};

// ---------------------------------------------------------------------------
// BGM — Beginning of Message
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BgmSegment {
	pub meldecode: Meldecode,
	pub dokumentennummer: String,
	pub nachrichtenfunktion: Nachrichtenfunktion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Meldecode {
	E01,
	E02,
	E03,
	E04,
	Z08,
	Z09,
	Z10,
	Z33,
	Z34,
}

impl Meldecode {
	pub fn as_str(self) -> &'static str {
		match self {
			Meldecode::E01 => "E01",
			Meldecode::E02 => "E02",
			Meldecode::E03 => "E03",
			Meldecode::E04 => "E04",
			Meldecode::Z08 => "Z08",
			Meldecode::Z09 => "Z09",
			Meldecode::Z10 => "Z10",
			Meldecode::Z33 => "Z33",
			Meldecode::Z34 => "Z34",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"E01" => Ok(Meldecode::E01),
			"E02" => Ok(Meldecode::E02),
			"E03" => Ok(Meldecode::E03),
			"E04" => Ok(Meldecode::E04),
			"Z08" => Ok(Meldecode::Z08),
			"Z09" => Ok(Meldecode::Z09),
			"Z10" => Ok(Meldecode::Z10),
			"Z33" => Ok(Meldecode::Z33),
			"Z34" => Ok(Meldecode::Z34),
			_ => Err(ParseError::InvalidQualifier(format!("unknown Meldecode: {s}"))),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nachrichtenfunktion {
	Original,
	Ersatz,
	Storno,
}

impl Nachrichtenfunktion {
	pub fn code(self) -> &'static str {
		match self {
			Nachrichtenfunktion::Original => "9",
			Nachrichtenfunktion::Ersatz => "5",
			Nachrichtenfunktion::Storno => "1",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"9" => Ok(Nachrichtenfunktion::Original),
			"5" => Ok(Nachrichtenfunktion::Ersatz),
			"1" => Ok(Nachrichtenfunktion::Storno),
			_ => Err(ParseError::InvalidQualifier(format!(
				"unknown Nachrichtenfunktion: {s}"
			))),
		}
	}
}

// ---------------------------------------------------------------------------
// DTM — Date/Time/Period
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DtmSegment {
	pub qualifier: DtmQualifier,
	pub datum: NaiveDate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DtmQualifier {
	Nachrichtendatum,
	Lieferbeginn,
	Lieferende,
	Abrechnungsbeginn,
	Abrechnungsende,
	Bilanzierungsdatum,
}

impl DtmQualifier {
	pub fn code(self) -> &'static str {
		match self {
			DtmQualifier::Nachrichtendatum => "137",
			DtmQualifier::Lieferbeginn => "92",
			DtmQualifier::Lieferende => "616",
			DtmQualifier::Abrechnungsbeginn => "324",
			DtmQualifier::Abrechnungsende => "206",
			DtmQualifier::Bilanzierungsdatum => "735",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"137" => Ok(DtmQualifier::Nachrichtendatum),
			"92" => Ok(DtmQualifier::Lieferbeginn),
			"616" => Ok(DtmQualifier::Lieferende),
			"324" => Ok(DtmQualifier::Abrechnungsbeginn),
			"206" => Ok(DtmQualifier::Abrechnungsende),
			"735" => Ok(DtmQualifier::Bilanzierungsdatum),
			_ => Err(ParseError::InvalidQualifier(format!("unknown DTM qualifier: {s}"))),
		}
	}
}

// ---------------------------------------------------------------------------
// NAD — Name and Address
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NadSegment {
	pub qualifier: NadQualifier,
	pub mp_id: String,
	pub codeliste: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NadQualifier {
	Absender,
	Empfaenger,
	Netzbetreiber,
	Lieferant,
	Messstellenbetreiber,
}

impl NadQualifier {
	pub fn code(self) -> &'static str {
		match self {
			NadQualifier::Absender => "MS",
			NadQualifier::Empfaenger => "MR",
			NadQualifier::Netzbetreiber => "Z06",
			NadQualifier::Lieferant => "ZD4",
			NadQualifier::Messstellenbetreiber => "DP",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"MS" => Ok(NadQualifier::Absender),
			"MR" => Ok(NadQualifier::Empfaenger),
			"Z06" => Ok(NadQualifier::Netzbetreiber),
			"ZD4" => Ok(NadQualifier::Lieferant),
			"DP" => Ok(NadQualifier::Messstellenbetreiber),
			_ => Err(ParseError::InvalidQualifier(format!("unknown NAD qualifier: {s}"))),
		}
	}
}

// ---------------------------------------------------------------------------
// IDE — Identification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdeSegment {
	pub qualifier: IdeQualifier,
	pub identifikator: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdeQualifier {
	Marktlokation,
	Messlokation,
}

impl IdeQualifier {
	pub fn code(self) -> &'static str {
		match self {
			IdeQualifier::Marktlokation => "24",
			IdeQualifier::Messlokation => "25",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"24" => Ok(IdeQualifier::Marktlokation),
			"25" => Ok(IdeQualifier::Messlokation),
			_ => Err(ParseError::InvalidQualifier(format!("unknown IDE qualifier: {s}"))),
		}
	}
}

// ---------------------------------------------------------------------------
// RFF — Reference
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RffSegment {
	pub qualifier: RffQualifier,
	pub referenz: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RffQualifier {
	PruefIdentifikator,
	Vorgangsnummer,
	Rechnungsnummer,
}

impl RffQualifier {
	pub fn code(self) -> &'static str {
		match self {
			RffQualifier::PruefIdentifikator => "Z13",
			RffQualifier::Vorgangsnummer => "ACW",
			RffQualifier::Rechnungsnummer => "IV",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"Z13" => Ok(RffQualifier::PruefIdentifikator),
			"ACW" => Ok(RffQualifier::Vorgangsnummer),
			"IV" => Ok(RffQualifier::Rechnungsnummer),
			_ => Err(ParseError::InvalidQualifier(format!("unknown RFF qualifier: {s}"))),
		}
	}
}

// ---------------------------------------------------------------------------
// LOC — Place/Location
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocSegment {
	pub qualifier: LocQualifier,
	pub ort: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocQualifier {
	Marktlokation,
	Messlokation,
}

impl LocQualifier {
	pub fn code(self) -> &'static str {
		match self {
			LocQualifier::Marktlokation => "172",
			LocQualifier::Messlokation => "Z16",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"172" => Ok(LocQualifier::Marktlokation),
			"Z16" => Ok(LocQualifier::Messlokation),
			_ => Err(ParseError::InvalidQualifier(format!("unknown LOC qualifier: {s}"))),
		}
	}
}

// ---------------------------------------------------------------------------
// QTY — Quantity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QtySegment {
	pub qualifier: QtyQualifier,
	pub menge: f64,
	pub einheit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QtyQualifier {
	Verbrauch,
	Einspeisung,
}

impl QtyQualifier {
	pub fn code(self) -> &'static str {
		match self {
			QtyQualifier::Verbrauch => "220",
			QtyQualifier::Einspeisung => "221",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"220" => Ok(QtyQualifier::Verbrauch),
			"221" => Ok(QtyQualifier::Einspeisung),
			_ => Err(ParseError::InvalidQualifier(format!("unknown QTY qualifier: {s}"))),
		}
	}
}

// ---------------------------------------------------------------------------
// MOA — Monetary Amount
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoaSegment {
	pub qualifier: MoaQualifier,
	pub betrag: f64,
	pub waehrung: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoaQualifier {
	Gesamtbetrag,
	Steuerbetrag,
	Nettobetrag,
}

impl MoaQualifier {
	pub fn code(self) -> &'static str {
		match self {
			MoaQualifier::Gesamtbetrag => "77",
			MoaQualifier::Steuerbetrag => "176",
			MoaQualifier::Nettobetrag => "125",
		}
	}

	pub fn parse(s: &str) -> Result<Self, ParseError> {
		match s {
			"77" => Ok(MoaQualifier::Gesamtbetrag),
			"176" => Ok(MoaQualifier::Steuerbetrag),
			"125" => Ok(MoaQualifier::Nettobetrag),
			_ => Err(ParseError::InvalidQualifier(format!("unknown MOA qualifier: {s}"))),
		}
	}
}

// ---------------------------------------------------------------------------
// STS — Status
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StsSegment {
	pub status_code: String,
	pub beschreibung: Option<String>,
}

// ---------------------------------------------------------------------------
// CCI — Characteristic/Class ID
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CciSegment {
	pub merkmal: String,
	pub wert: String,
}

// ---------------------------------------------------------------------------
// TypedEdifactNachricht — full typed message
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedEdifactNachricht {
	pub bgm: BgmSegment,
	pub dtm: Vec<DtmSegment>,
	pub nad: Vec<NadSegment>,
	pub ide: Vec<IdeSegment>,
	pub rff: Vec<RffSegment>,
	pub loc: Vec<LocSegment>,
	pub qty: Vec<QtySegment>,
	pub moa: Vec<MoaSegment>,
	pub sts: Vec<StsSegment>,
	pub cci: Vec<CciSegment>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Get first component of element at index, or empty string.
fn component(seg: &Segment, element_idx: usize, component_idx: usize) -> String {
	seg.elements
		.get(element_idx)
		.and_then(|e| e.components.get(component_idx))
		.cloned()
		.unwrap_or_default()
}

/// Parse EDIFACT date format 102 (CCYYMMDD) into NaiveDate.
fn parse_date_102(s: &str) -> Result<NaiveDate, ParseError> {
	NaiveDate::parse_from_str(s, "%Y%m%d")
		.map_err(|_| ParseError::InvalidDate(s.to_string()))
}

// ---------------------------------------------------------------------------
// Per-segment parsers (raw Segment → typed segment)
// ---------------------------------------------------------------------------

fn parse_bgm(seg: &Segment) -> Result<BgmSegment, ParseError> {
	let meldecode_str = component(seg, 0, 0);
	let dokumentennummer = component(seg, 1, 0);
	let funktion_str = component(seg, 2, 0);

	Ok(BgmSegment {
		meldecode: Meldecode::parse(&meldecode_str)?,
		dokumentennummer,
		nachrichtenfunktion: Nachrichtenfunktion::parse(&funktion_str)?,
	})
}

fn parse_dtm(seg: &Segment) -> Result<DtmSegment, ParseError> {
	let qualifier_str = component(seg, 0, 0);
	let date_str = component(seg, 0, 1);

	Ok(DtmSegment {
		qualifier: DtmQualifier::parse(&qualifier_str)?,
		datum: parse_date_102(&date_str)?,
	})
}

fn parse_nad(seg: &Segment) -> Result<NadSegment, ParseError> {
	let qualifier_str = component(seg, 0, 0);
	let mp_id = component(seg, 1, 0);
	let codeliste = component(seg, 1, 2);

	Ok(NadSegment {
		qualifier: NadQualifier::parse(&qualifier_str)?,
		mp_id,
		codeliste,
	})
}

fn parse_ide(seg: &Segment) -> Result<IdeSegment, ParseError> {
	let qualifier_str = component(seg, 0, 0);
	let identifikator = component(seg, 1, 0);

	Ok(IdeSegment {
		qualifier: IdeQualifier::parse(&qualifier_str)?,
		identifikator,
	})
}

fn parse_rff(seg: &Segment) -> Result<RffSegment, ParseError> {
	let qualifier_str = component(seg, 0, 0);
	let referenz = component(seg, 0, 1);

	Ok(RffSegment {
		qualifier: RffQualifier::parse(&qualifier_str)?,
		referenz,
	})
}

fn parse_loc(seg: &Segment) -> Result<LocSegment, ParseError> {
	let qualifier_str = component(seg, 0, 0);
	let ort = component(seg, 1, 0);

	Ok(LocSegment {
		qualifier: LocQualifier::parse(&qualifier_str)?,
		ort,
	})
}

fn parse_qty(seg: &Segment) -> Result<QtySegment, ParseError> {
	let qualifier_str = component(seg, 0, 0);
	let menge_str = component(seg, 0, 1);
	let einheit = component(seg, 0, 2);

	let menge: f64 = menge_str
		.parse()
		.map_err(|_| ParseError::InvalidSegment(format!("invalid QTY amount: {menge_str}")))?;

	Ok(QtySegment {
		qualifier: QtyQualifier::parse(&qualifier_str)?,
		menge,
		einheit,
	})
}

fn parse_moa(seg: &Segment) -> Result<MoaSegment, ParseError> {
	let qualifier_str = component(seg, 0, 0);
	let betrag_str = component(seg, 0, 1);
	let waehrung = component(seg, 0, 2);

	let betrag: f64 = betrag_str
		.parse()
		.map_err(|_| ParseError::InvalidSegment(format!("invalid MOA amount: {betrag_str}")))?;

	Ok(MoaSegment {
		qualifier: MoaQualifier::parse(&qualifier_str)?,
		betrag,
		waehrung,
	})
}

fn parse_sts(seg: &Segment) -> Result<StsSegment, ParseError> {
	let status_code = component(seg, 0, 0);
	let beschreibung = seg
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.filter(|s| !s.is_empty())
		.cloned();

	Ok(StsSegment {
		status_code,
		beschreibung,
	})
}

fn parse_cci(seg: &Segment) -> Result<CciSegment, ParseError> {
	let merkmal = component(seg, 0, 0);
	let wert = component(seg, 1, 0);

	Ok(CciSegment { merkmal, wert })
}

// ---------------------------------------------------------------------------
// Typed → raw conversion
// ---------------------------------------------------------------------------

impl BgmSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "BGM".to_string(),
			elements: vec![
				Element { components: vec![self.meldecode.as_str().to_string()] },
				Element { components: vec![self.dokumentennummer.clone()] },
				Element { components: vec![self.nachrichtenfunktion.code().to_string()] },
			],
		}
	}
}

impl DtmSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "DTM".to_string(),
			elements: vec![Element {
				components: vec![
					self.qualifier.code().to_string(),
					format!(
						"{:04}{:02}{:02}",
						self.datum.year(),
						self.datum.month(),
						self.datum.day()
					),
					"102".to_string(),
				],
			}],
		}
	}
}

impl NadSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "NAD".to_string(),
			elements: vec![
				Element { components: vec![self.qualifier.code().to_string()] },
				Element {
					components: vec![
						self.mp_id.clone(),
						String::new(),
						self.codeliste.clone(),
					],
				},
			],
		}
	}
}

impl IdeSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "IDE".to_string(),
			elements: vec![
				Element { components: vec![self.qualifier.code().to_string()] },
				Element { components: vec![self.identifikator.clone()] },
			],
		}
	}
}

impl RffSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "RFF".to_string(),
			elements: vec![Element {
				components: vec![
					self.qualifier.code().to_string(),
					self.referenz.clone(),
				],
			}],
		}
	}
}

impl LocSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "LOC".to_string(),
			elements: vec![
				Element { components: vec![self.qualifier.code().to_string()] },
				Element { components: vec![self.ort.clone()] },
			],
		}
	}
}

impl QtySegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "QTY".to_string(),
			elements: vec![Element {
				components: vec![
					self.qualifier.code().to_string(),
					self.menge.to_string(),
					self.einheit.clone(),
				],
			}],
		}
	}
}

impl MoaSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "MOA".to_string(),
			elements: vec![Element {
				components: vec![
					self.qualifier.code().to_string(),
					self.betrag.to_string(),
					self.waehrung.clone(),
				],
			}],
		}
	}
}

impl StsSegment {
	pub fn to_raw_segment(&self) -> Segment {
		let mut elements = vec![Element {
			components: vec![self.status_code.clone()],
		}];
		if let Some(ref beschr) = self.beschreibung {
			elements.push(Element {
				components: vec![beschr.clone()],
			});
		}
		Segment {
			tag: "STS".to_string(),
			elements,
		}
	}
}

impl CciSegment {
	pub fn to_raw_segment(&self) -> Segment {
		Segment {
			tag: "CCI".to_string(),
			elements: vec![
				Element { components: vec![self.merkmal.clone()] },
				Element { components: vec![self.wert.clone()] },
			],
		}
	}
}

impl TypedEdifactNachricht {
	pub fn to_raw_segments(&self) -> Vec<Segment> {
		let mut segs = Vec::new();
		segs.push(self.bgm.to_raw_segment());
		for dtm in &self.dtm {
			segs.push(dtm.to_raw_segment());
		}
		for nad in &self.nad {
			segs.push(nad.to_raw_segment());
		}
		for ide in &self.ide {
			segs.push(ide.to_raw_segment());
		}
		for rff in &self.rff {
			segs.push(rff.to_raw_segment());
		}
		for loc in &self.loc {
			segs.push(loc.to_raw_segment());
		}
		for qty in &self.qty {
			segs.push(qty.to_raw_segment());
		}
		for moa in &self.moa {
			segs.push(moa.to_raw_segment());
		}
		for sts in &self.sts {
			segs.push(sts.to_raw_segment());
		}
		for cci in &self.cci {
			segs.push(cci.to_raw_segment());
		}
		segs
	}
}

// ---------------------------------------------------------------------------
// from_raw_segments: raw → typed
// ---------------------------------------------------------------------------

pub fn from_raw_segments(segments: &[Segment]) -> Result<TypedEdifactNachricht, ParseError> {
	let mut bgm: Option<BgmSegment> = None;
	let mut dtm = Vec::new();
	let mut nad = Vec::new();
	let mut ide = Vec::new();
	let mut rff = Vec::new();
	let mut loc = Vec::new();
	let mut qty = Vec::new();
	let mut moa = Vec::new();
	let mut sts = Vec::new();
	let mut cci = Vec::new();

	for seg in segments {
		match seg.tag.as_str() {
			"BGM" => bgm = Some(parse_bgm(seg)?),
			"DTM" => dtm.push(parse_dtm(seg)?),
			"NAD" => nad.push(parse_nad(seg)?),
			"IDE" => ide.push(parse_ide(seg)?),
			"RFF" => rff.push(parse_rff(seg)?),
			"LOC" => loc.push(parse_loc(seg)?),
			"QTY" => qty.push(parse_qty(seg)?),
			"MOA" => moa.push(parse_moa(seg)?),
			"STS" => sts.push(parse_sts(seg)?),
			"CCI" => cci.push(parse_cci(seg)?),
			// Skip unknown segments (UNH, UNT, UNB, UNZ, etc.)
			_ => {}
		}
	}

	let bgm = bgm.ok_or_else(|| ParseError::MissingSegment("BGM".to_string()))?;

	Ok(TypedEdifactNachricht {
		bgm,
		dtm,
		nad,
		ide,
		rff,
		loc,
		qty,
		moa,
		sts,
		cci,
	})
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	use crate::edifact::parser::parse_segments;

	#[test]
	fn parse_bgm_segment() {
		let segments = parse_segments("BGM+E01+12345+9'").unwrap();
		let typed = from_raw_segments(&segments).unwrap();
		assert_eq!(typed.bgm.meldecode, Meldecode::E01);
		assert_eq!(typed.bgm.dokumentennummer, "12345");
		assert_eq!(typed.bgm.nachrichtenfunktion, Nachrichtenfunktion::Original);
	}

	#[test]
	fn parse_dtm_segment() {
		let segments = parse_segments("BGM+E01+1+9'DTM+137:20250701:102'").unwrap();
		let typed = from_raw_segments(&segments).unwrap();
		assert_eq!(typed.dtm.len(), 1);
		assert_eq!(typed.dtm[0].qualifier, DtmQualifier::Nachrichtendatum);
		assert_eq!(typed.dtm[0].datum, NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());
	}

	#[test]
	fn parse_nad_segment() {
		let segments =
			parse_segments("BGM+E01+1+9'NAD+MS+9900000000003::293'").unwrap();
		let typed = from_raw_segments(&segments).unwrap();
		assert_eq!(typed.nad.len(), 1);
		assert_eq!(typed.nad[0].qualifier, NadQualifier::Absender);
		assert_eq!(typed.nad[0].mp_id, "9900000000003");
		assert_eq!(typed.nad[0].codeliste, "293");
	}

	#[test]
	fn parse_rff_segment() {
		let segments = parse_segments("BGM+E01+1+9'RFF+Z13:44001'").unwrap();
		let typed = from_raw_segments(&segments).unwrap();
		assert_eq!(typed.rff.len(), 1);
		assert_eq!(typed.rff[0].qualifier, RffQualifier::PruefIdentifikator);
		assert_eq!(typed.rff[0].referenz, "44001");
	}

	#[test]
	fn round_trip_typed_to_raw_and_back() {
		let original = TypedEdifactNachricht {
			bgm: BgmSegment {
				meldecode: Meldecode::E02,
				dokumentennummer: "DOC-99".to_string(),
				nachrichtenfunktion: Nachrichtenfunktion::Storno,
			},
			dtm: vec![DtmSegment {
				qualifier: DtmQualifier::Lieferbeginn,
				datum: NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
			}],
			nad: vec![NadSegment {
				qualifier: NadQualifier::Empfaenger,
				mp_id: "9900000000010".to_string(),
				codeliste: "293".to_string(),
			}],
			ide: vec![IdeSegment {
				qualifier: IdeQualifier::Marktlokation,
				identifikator: "DE000111222333".to_string(),
			}],
			rff: vec![RffSegment {
				qualifier: RffQualifier::Vorgangsnummer,
				referenz: "VG-001".to_string(),
			}],
			loc: vec![LocSegment {
				qualifier: LocQualifier::Messlokation,
				ort: "DE000444555666".to_string(),
			}],
			qty: vec![QtySegment {
				qualifier: QtyQualifier::Verbrauch,
				menge: 1234.5,
				einheit: "KWH".to_string(),
			}],
			moa: vec![MoaSegment {
				qualifier: MoaQualifier::Nettobetrag,
				betrag: 99.99,
				waehrung: "EUR".to_string(),
			}],
			sts: vec![StsSegment {
				status_code: "7".to_string(),
				beschreibung: Some("Zustimmung".to_string()),
			}],
			cci: vec![CciSegment {
				merkmal: "Z01".to_string(),
				wert: "ETZ".to_string(),
			}],
		};

		let raw = original.to_raw_segments();
		let parsed_back = from_raw_segments(&raw).unwrap();
		assert_eq!(original, parsed_back);
	}

	#[test]
	fn missing_bgm_returns_error() {
		let segments = parse_segments("DTM+137:20250701:102'").unwrap();
		let result = from_raw_segments(&segments);
		assert!(matches!(result, Err(ParseError::MissingSegment(_))));
	}

	#[test]
	fn unknown_meldecode_returns_error() {
		let segments = parse_segments("BGM+X99+1+9'").unwrap();
		let result = from_raw_segments(&segments);
		assert!(matches!(result, Err(ParseError::InvalidQualifier(_))));
	}

	#[test]
	fn invalid_date_returns_error() {
		let segments = parse_segments("BGM+E01+1+9'DTM+137:99999999:102'").unwrap();
		let result = from_raw_segments(&segments);
		assert!(matches!(result, Err(ParseError::InvalidDate(_))));
	}

	#[test]
	fn skips_unknown_segment_tags() {
		let segments =
			parse_segments("BGM+E01+1+9'UNH+1+UTILMD:D:11A:UN'FTX+free text'").unwrap();
		let typed = from_raw_segments(&segments).unwrap();
		assert_eq!(typed.bgm.meldecode, Meldecode::E01);
		// UNH and FTX are silently skipped
	}
}

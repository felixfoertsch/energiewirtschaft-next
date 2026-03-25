use serde::{Deserialize, Serialize};

/// A flat AHB (Anwendungshandbuch) — defines which fields are
/// Muss/Soll/Kann for a specific Prüfidentifikator.
/// Loaded from Hochfrequenz machine-readable AHB JSON files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ahb {
	pub lines: Vec<AhbLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AhbLine {
	pub ahb_expression: String,
	pub conditions: String,
	pub data_element: Option<String>,
	pub guid: String,
	pub index: u32,
	pub name: String,
	pub section_name: String,
	pub segment_code: Option<String>,
	pub segment_group_key: Option<String>,
	pub segment_id: Option<String>,
	pub value_pool_entry: Option<String>,
}

/// Classification of an AHB field requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feldanforderung {
	Muss,
	Soll,
	Kann,
	/// Required but context-dependent
	X,
	/// Conditional (references a condition number)
	Bedingt(u32),
}

impl AhbLine {
	/// Parse the ahb_expression into a typed Feldanforderung.
	pub fn anforderung(&self) -> Feldanforderung {
		match self.ahb_expression.as_str() {
			"Muss" => Feldanforderung::Muss,
			"Soll" => Feldanforderung::Soll,
			"Kann" => Feldanforderung::Kann,
			"X" => Feldanforderung::X,
			s if s.starts_with("Muss [")
				|| s.starts_with("Soll [")
				|| s.starts_with("Kann [") =>
			{
				// Conditional: "Muss [123]" etc.
				Feldanforderung::Bedingt(0) // simplified
			}
			_ => Feldanforderung::Kann,
		}
	}
}

impl Ahb {
	/// Load from JSON string.
	pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
		serde_json::from_str(json)
	}

	/// Get all Muss-fields (mandatory fields).
	pub fn muss_felder(&self) -> Vec<&AhbLine> {
		self.lines
			.iter()
			.filter(|l| l.anforderung() == Feldanforderung::Muss)
			.collect()
	}

	/// Get all lines for a specific segment.
	pub fn segment_lines(&self, segment_code: &str) -> Vec<&AhbLine> {
		self.lines
			.iter()
			.filter(|l| l.segment_code.as_deref() == Some(segment_code))
			.collect()
	}

	/// Get all unique segment codes used in this AHB.
	pub fn segmente(&self) -> Vec<String> {
		self.lines
			.iter()
			.filter_map(|l| l.segment_code.clone())
			.collect::<std::collections::BTreeSet<_>>()
			.into_iter()
			.collect()
	}

	/// Validate that all Muss-fields have values.
	/// Takes a set of (segment_code, data_element) tuples representing present fields.
	/// Returns list of missing mandatory fields.
	pub fn validate_muss_felder(
		&self,
		vorhanden: &std::collections::HashSet<(String, String)>,
	) -> Vec<&AhbLine> {
		self.muss_felder()
			.into_iter()
			.filter(|line| {
				if let (Some(sc), Some(de)) = (&line.segment_code, &line.data_element) {
					!vorhanden.contains(&(sc.clone(), de.clone()))
				} else {
					false // segment headers don't need value validation
				}
			})
			.collect()
	}
}

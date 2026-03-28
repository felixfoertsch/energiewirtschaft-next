use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::Deserialize;

// ---------------------------------------------------------------------------
// AHB types (from Hochfrequenz machine-readable_anwendungshandbuecher)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct AhbMeta {
	pub description: String,
	pub direction: String,
	pub maus_version: String,
	pub pruefidentifikator: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AhbZeile {
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

#[derive(Debug, Clone, Deserialize)]
pub struct AhbDokument {
	pub meta: AhbMeta,
	pub lines: Vec<AhbZeile>,
}

// ---------------------------------------------------------------------------
// EBD types (from Hochfrequenz machine-readable_entscheidungsbaumdiagramme)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct EbdMetadata {
	pub chapter: String,
	pub ebd_code: String,
	pub ebd_name: String,
	pub link: Option<String>,
	pub note: Option<String>,
	pub release_information: Option<EbdReleaseInformation>,
	pub remark: Option<String>,
	pub role: String,
	pub section: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EbdReleaseInformation {
	pub ebdamame_version: Option<String>,
	pub original_release_date: Option<String>,
	pub rebdhuhn_version: Option<String>,
	pub release_date: Option<String>,
	pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EbdPruefErgebnis {
	pub result: bool,
	pub subsequent_step_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EbdUnterzeile {
	pub check_result: EbdPruefErgebnis,
	#[serde(default)]
	pub ebd_references: Vec<serde_json::Value>,
	pub note: Option<String>,
	pub result_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EbdZeile {
	pub description: String,
	pub step_number: String,
	pub sub_rows: Vec<EbdUnterzeile>,
	pub use_cases: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EbdDokument {
	pub metadata: EbdMetadata,
	pub multi_step_instructions: Option<serde_json::Value>,
	pub rows: Vec<EbdZeile>,
}

// ---------------------------------------------------------------------------
// Referenzdaten loader with caching
// ---------------------------------------------------------------------------

pub struct Referenzdaten {
	basis: PathBuf,
	ahb_version: String,
	ebd_version: String,
	ahb_cache: Mutex<HashMap<(String, String), AhbDokument>>,
	ebd_cache: Mutex<HashMap<String, EbdDokument>>,
}

impl Referenzdaten {
	/// Create a new reference data loader.
	///
	/// `basis` is the directory containing the `ahb/` and `ebd/` subdirectories.
	/// `ahb_version` and `ebd_version` are format version strings like `"FV2504"`.
	pub fn laden(basis: impl AsRef<Path>, ahb_version: &str, ebd_version: &str) -> Self {
		Self {
			basis: basis.as_ref().to_path_buf(),
			ahb_version: ahb_version.to_string(),
			ebd_version: ebd_version.to_string(),
			ahb_cache: Mutex::new(HashMap::new()),
			ebd_cache: Mutex::new(HashMap::new()),
		}
	}

	/// Load an AHB document by message type and pruefidentifikator.
	///
	/// Returns `None` if the file does not exist.
	pub fn ahb(&self, nachrichtentyp: &str, pruefidentifikator: &str) -> Option<AhbDokument> {
		let key = (nachrichtentyp.to_string(), pruefidentifikator.to_string());

		// check cache
		{
			let cache = self.ahb_cache.lock().unwrap();
			if let Some(dok) = cache.get(&key) {
				return Some(dok.clone());
			}
		}

		// build path: basis/ahb/{version}/{nachrichtentyp}/{pi}.json
		let pfad = self
			.basis
			.join("ahb")
			.join(&self.ahb_version)
			.join(nachrichtentyp)
			.join(format!("{pruefidentifikator}.json"));

		let inhalt = fs::read_to_string(&pfad).ok()?;
		let dok: AhbDokument = serde_json::from_str(&inhalt).ok()?;

		let mut cache = self.ahb_cache.lock().unwrap();
		cache.insert(key, dok.clone());
		Some(dok)
	}

	/// Load an EBD document by EBD code (e.g., `"E_0003"`).
	///
	/// Returns `None` if the file does not exist.
	pub fn ebd(&self, ebd_code: &str) -> Option<EbdDokument> {
		// check cache
		{
			let cache = self.ebd_cache.lock().unwrap();
			if let Some(dok) = cache.get(ebd_code) {
				return Some(dok.clone());
			}
		}

		// build path: basis/ebd/{version}/{ebd_code}.json
		let pfad = self
			.basis
			.join("ebd")
			.join(&self.ebd_version)
			.join(format!("{ebd_code}.json"));

		let inhalt = fs::read_to_string(&pfad).ok()?;
		let dok: EbdDokument = serde_json::from_str(&inhalt).ok()?;

		let mut cache = self.ebd_cache.lock().unwrap();
		cache.insert(ebd_code.to_string(), dok.clone());
		Some(dok)
	}

	/// List all available message types (subdirectories under ahb/{version}/).
	pub fn nachrichtentypen(&self) -> Vec<String> {
		let dir = self.basis.join("ahb").join(&self.ahb_version);
		let Ok(entries) = fs::read_dir(&dir) else {
			return Vec::new();
		};

		let mut typen: Vec<String> = entries
			.filter_map(|e| e.ok())
			.filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
			.filter_map(|e| e.file_name().into_string().ok())
			.collect();
		typen.sort();
		typen
	}

	/// List all available pruefidentifikatoren for a message type.
	pub fn pruefidentifikatoren(&self, nachrichtentyp: &str) -> Vec<String> {
		let dir = self
			.basis
			.join("ahb")
			.join(&self.ahb_version)
			.join(nachrichtentyp);
		let Ok(entries) = fs::read_dir(&dir) else {
			return Vec::new();
		};

		let mut pis: Vec<String> = entries
			.filter_map(|e| e.ok())
			.filter_map(|e| {
				let name = e.file_name().into_string().ok()?;
				name.strip_suffix(".json").map(|s| s.to_string())
			})
			.collect();
		pis.sort();
		pis
	}

	/// List all available EBD codes.
	pub fn ebd_codes(&self) -> Vec<String> {
		let dir = self.basis.join("ebd").join(&self.ebd_version);
		let Ok(entries) = fs::read_dir(&dir) else {
			return Vec::new();
		};

		let mut codes: Vec<String> = entries
			.filter_map(|e| e.ok())
			.filter_map(|e| {
				let name = e.file_name().into_string().ok()?;
				name.strip_suffix(".json").map(|s| s.to_string())
			})
			.collect();
		codes.sort();
		codes
	}
}

use serde::{Deserialize, Serialize};

/// An Entscheidungsbaumdiagramm — a decision tree that defines the
/// APERAK validation logic for a specific process step.
/// Loaded from Hochfrequenz machine-readable EBD JSON files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ebd {
	pub metadata: EbdMetadata,
	pub multi_step_instructions: Option<Vec<MultiStepInstruction>>,
	pub rows: Vec<EbdRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EbdMetadata {
	pub chapter: String,
	pub ebd_code: String,
	pub ebd_name: String,
	pub remark: Option<String>,
	pub role: String,
	pub section: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiStepInstruction {
	pub step_number: String,
	pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EbdRow {
	pub description: String,
	pub step_number: String,
	pub sub_rows: Vec<EbdSubRow>,
	pub use_cases: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EbdSubRow {
	pub check_result: CheckResult,
	pub note: Option<String>,
	pub result_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckResult {
	pub result: bool,
	pub subsequent_step_number: Option<String>,
}

impl Ebd {
	/// Load an EBD from a JSON string.
	pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
		serde_json::from_str(json)
	}

	/// Evaluate the decision tree for a given set of check results.
	/// Returns the final result code(s) that apply.
	pub fn evaluate(&self, checks: &dyn Fn(&str) -> bool) -> Vec<String> {
		let mut current_step = self.rows.first().map(|r| r.step_number.clone());
		let mut result_codes = Vec::new();

		while let Some(step_num) = &current_step {
			if step_num == "Ende" {
				break;
			}
			let row = self.rows.iter().find(|r| &r.step_number == step_num);
			match row {
				Some(row) => {
					let check_result = checks(&row.description);
					let sub_row = row
						.sub_rows
						.iter()
						.find(|sr| sr.check_result.result == check_result);
					match sub_row {
						Some(sr) => {
							if let Some(code) = &sr.result_code {
								result_codes.push(code.clone());
							}
							current_step = sr.check_result.subsequent_step_number.clone();
						}
						None => break,
					}
				}
				None => break,
			}
		}

		result_codes
	}
}

use serde::{Deserialize, Serialize};

use crate::ids::MarktpartnerId;

/// IFTSTA status message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IftstaStatusmeldung {
	pub referenz_nachricht: String,
	pub status_code: String,
	pub beschreibung: String,
}

/// PARTIN market partner master data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartinMarktpartner {
	pub mp_id: MarktpartnerId,
	pub name: String,
	pub rolle: String,
}

/// UTILTS calculation formulas / metering time definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtiltsZaehlzeitdefinition {
	pub formel_id: String,
	pub bezeichnung: String,
	pub zeitreihen_typ: String,
}

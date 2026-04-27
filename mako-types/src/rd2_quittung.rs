use chrono::{DateTime, FixedOffset};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ids::MarktpartnerId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AcknowledgementTyp {
	Positiv,
	Negativ,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AcknowledgementDocument {
	pub receiver_mrid: MarktpartnerId,
	pub sender_mrid: MarktpartnerId,
	pub original_message_mrid: String,
	pub received_at: DateTime<FixedOffset>,
	pub ack_typ: AcknowledgementTyp,
	pub reason: Option<String>,
}

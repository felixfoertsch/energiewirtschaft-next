use serde::{Deserialize, Serialize};

use crate::gpke_nachrichten::*;
use crate::ids::MarktpartnerId;
use crate::rolle::MarktRolle;

/// Envelope for any MaKo message, carrying routing info and typed payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nachricht {
	pub absender: MarktpartnerId,
	pub absender_rolle: MarktRolle,
	pub empfaenger: MarktpartnerId,
	pub empfaenger_rolle: MarktRolle,
	pub payload: NachrichtenPayload,
}

/// Typed payload — one variant per concrete message type.
/// Extended as new message types are implemented.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NachrichtenPayload {
	UtilmdAnmeldung(UtilmdAnmeldung),
	UtilmdBestaetigung(UtilmdBestaetigung),
	UtilmdAbmeldung(UtilmdAbmeldung),
	UtilmdAblehnung(UtilmdAblehnung),
	UtilmdZuordnung(UtilmdZuordnung),
	UtilmdLieferendeAbmeldung(UtilmdLieferendeAbmeldung),
	UtilmdLieferendeBestaetigung(UtilmdLieferendeBestaetigung),
	MsconsSchlussturnusmesswert(MsconsSchlussturnusmesswert),
	MsconsLastgang(MsconsLastgang),
	UtilmdStammdatenaenderung(UtilmdStammdatenaenderung),
	UtilmdZuordnungsliste(UtilmdZuordnungsliste),
	UtilmdGeschaeftsdatenanfrage(UtilmdGeschaeftsdatenanfrage),
	UtilmdGeschaeftsdatenantwort(UtilmdGeschaeftsdatenantwort),
}

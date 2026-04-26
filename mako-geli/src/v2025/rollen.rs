use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenPfad = &'static [MarktRolle];

pub const LIEFERANTENWECHSEL_GAS_ANMELDUNG: RollenPfad = &[LieferantNeu, Netzbetreiber];
pub const LIEFERANTENWECHSEL_GAS_BESTAETIGUNG_AN_LFN: RollenPfad = &[Netzbetreiber, LieferantNeu];
pub const LIEFERANTENWECHSEL_GAS_INFORMATION_AN_LFA: RollenPfad = &[Netzbetreiber, LieferantAlt];
pub const ERSATZ_GRUNDVERSORGUNG_GAS_ANMELDUNG: RollenPfad =
	&[LieferantErsatzGrundversorgung, Netzbetreiber];
pub const GESCHAEFTSDATENANFRAGE_GAS_LIEFERANT: RollenPfad = &[Lieferant, Netzbetreiber];
pub const GMSB_AN_NETZBETREIBER: RollenPfad =
	&[GrundzustaendigerMessstellenbetreiber, Netzbetreiber];
pub const NETZBETREIBER_AN_GMSB: RollenPfad =
	&[Netzbetreiber, GrundzustaendigerMessstellenbetreiber];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollenpfade_sind_vollstaendiger_geli_kanon() {
		assert_eq!(
			LIEFERANTENWECHSEL_GAS_ANMELDUNG,
			&[LieferantNeu, Netzbetreiber]
		);
		assert_eq!(
			LIEFERANTENWECHSEL_GAS_BESTAETIGUNG_AN_LFN,
			&[Netzbetreiber, LieferantNeu]
		);
		assert_eq!(
			LIEFERANTENWECHSEL_GAS_INFORMATION_AN_LFA,
			&[Netzbetreiber, LieferantAlt]
		);
		assert_eq!(
			ERSATZ_GRUNDVERSORGUNG_GAS_ANMELDUNG,
			&[LieferantErsatzGrundversorgung, Netzbetreiber]
		);
		assert_eq!(
			GESCHAEFTSDATENANFRAGE_GAS_LIEFERANT,
			&[Lieferant, Netzbetreiber]
		);
		assert_eq!(
			GMSB_AN_NETZBETREIBER,
			&[GrundzustaendigerMessstellenbetreiber, Netzbetreiber]
		);
		assert_eq!(
			NETZBETREIBER_AN_GMSB,
			&[Netzbetreiber, GrundzustaendigerMessstellenbetreiber]
		);
	}
}

use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenPfad = &'static [MarktRolle];

pub const LIEFERANTENWECHSEL_ANMELDUNG: RollenPfad = &[LieferantNeu, Netzbetreiber];
pub const LIEFERANTENWECHSEL_BESTAETIGUNG_AN_LFN: RollenPfad = &[Netzbetreiber, LieferantNeu];
pub const LIEFERANTENWECHSEL_INFORMATION_AN_LFA: RollenPfad = &[Netzbetreiber, LieferantAlt];
pub const ERSATZ_GRUNDVERSORGUNG_ANMELDUNG: RollenPfad =
	&[LieferantErsatzGrundversorgung, Netzbetreiber];
pub const BILANZKREISZUORDNUNG: RollenPfad = &[
	Lieferant,
	Netzbetreiber,
	Bilanzkreisverantwortlicher,
	Bilanzkoordinator,
];
pub const GESCHAEFTSDATENANFRAGE_LIEFERANT: RollenPfad = &[Lieferant, Netzbetreiber];
pub const UEBERTRAGUNGSNETZ_BILANZIERUNG: RollenPfad = &[
	Netzbetreiber,
	Uebertragungsnetzbetreiber,
	Bilanzkreisverantwortlicher,
	Bilanzkoordinator,
];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollenpfade_sind_vollstaendiger_gpke_kanon() {
		let faelle: &[(&str, RollenPfad)] = &[
			("lieferantenwechsel_anmeldung", LIEFERANTENWECHSEL_ANMELDUNG),
			(
				"lieferantenwechsel_bestaetigung_an_lfn",
				LIEFERANTENWECHSEL_BESTAETIGUNG_AN_LFN,
			),
			(
				"lieferantenwechsel_information_an_lfa",
				LIEFERANTENWECHSEL_INFORMATION_AN_LFA,
			),
			(
				"ersatz_grundversorgung_anmeldung",
				ERSATZ_GRUNDVERSORGUNG_ANMELDUNG,
			),
			("bilanzkreiszuordnung", BILANZKREISZUORDNUNG),
			(
				"geschaeftsdatenanfrage_lieferant",
				GESCHAEFTSDATENANFRAGE_LIEFERANT,
			),
			(
				"uebertragungsnetz_bilanzierung",
				UEBERTRAGUNGSNETZ_BILANZIERUNG,
			),
		];

		assert_eq!(
			faelle[0],
			(
				"lieferantenwechsel_anmeldung",
				&[LieferantNeu, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[1],
			(
				"lieferantenwechsel_bestaetigung_an_lfn",
				&[Netzbetreiber, LieferantNeu][..]
			)
		);
		assert_eq!(
			faelle[2],
			(
				"lieferantenwechsel_information_an_lfa",
				&[Netzbetreiber, LieferantAlt][..]
			)
		);
		assert_eq!(
			faelle[3],
			(
				"ersatz_grundversorgung_anmeldung",
				&[LieferantErsatzGrundversorgung, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[4],
			(
				"bilanzkreiszuordnung",
				&[
					Lieferant,
					Netzbetreiber,
					Bilanzkreisverantwortlicher,
					Bilanzkoordinator,
				][..]
			)
		);
		assert_eq!(
			faelle[5],
			(
				"geschaeftsdatenanfrage_lieferant",
				&[Lieferant, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[6],
			(
				"uebertragungsnetz_bilanzierung",
				&[
					Netzbetreiber,
					Uebertragungsnetzbetreiber,
					Bilanzkreisverantwortlicher,
					Bilanzkoordinator,
				][..]
			)
		);
	}
}

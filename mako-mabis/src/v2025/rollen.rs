use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenPfad = &'static [MarktRolle];

pub const BILANZKREISZUORDNUNG: RollenPfad = &[Lieferant, Netzbetreiber];
pub const AGGREGIERTE_ZEITREIHEN_LASTGANG_SLP: RollenPfad =
	&[Netzbetreiber, Bilanzkreisverantwortlicher];
pub const MEHR_MINDERMENGENLISTE: RollenPfad = &[Netzbetreiber, Lieferant];
pub const CLEARINGLISTE: RollenPfad = &[Netzbetreiber, Lieferant];
pub const BKV_ANMELDUNG_BILANZKREIS: RollenPfad =
	&[Bilanzkreisverantwortlicher, Bilanzkoordinator];
pub const BIKO_BESTAETIGUNG: RollenPfad =
	&[Bilanzkoordinator, Bilanzkreisverantwortlicher];
pub const ANB_UEBERMITTLUNG_AN_BIKO: RollenPfad =
	&[Anschlussnetzbetreiber, Bilanzkoordinator];
pub const ANFNB_ANFORDERUNG: RollenPfad =
	&[AnfordernderNetzbetreiber, Anschlussnetzbetreiber];
pub const NETZBETREIBER_UEBERTRAGUNGSNETZ_AGGREGATE: RollenPfad =
	&[Netzbetreiber, Uebertragungsnetzbetreiber];
pub const BIKO_SALDENBERICHT: RollenPfad =
	&[Bilanzkoordinator, Uebertragungsnetzbetreiber];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollenpfade_sind_vollstaendiger_mabis_kanon() {
		let faelle: &[(&str, RollenPfad)] = &[
			("bilanzkreiszuordnung", BILANZKREISZUORDNUNG),
			(
				"aggregierte_zeitreihen_lastgang_slp",
				AGGREGIERTE_ZEITREIHEN_LASTGANG_SLP,
			),
			("mehr_mindermengenliste", MEHR_MINDERMENGENLISTE),
			("clearingliste", CLEARINGLISTE),
			("bkv_anmeldung_bilanzkreis", BKV_ANMELDUNG_BILANZKREIS),
			("biko_bestaetigung", BIKO_BESTAETIGUNG),
			("anb_uebermittlung_an_biko", ANB_UEBERMITTLUNG_AN_BIKO),
			("anfnb_anforderung", ANFNB_ANFORDERUNG),
			(
				"netzbetreiber_uebertragungsnetz_aggregate",
				NETZBETREIBER_UEBERTRAGUNGSNETZ_AGGREGATE,
			),
			("biko_saldenbericht", BIKO_SALDENBERICHT),
		];

		assert_eq!(
			faelle[0],
			("bilanzkreiszuordnung", &[Lieferant, Netzbetreiber][..])
		);
		assert_eq!(
			faelle[1],
			(
				"aggregierte_zeitreihen_lastgang_slp",
				&[Netzbetreiber, Bilanzkreisverantwortlicher][..]
			)
		);
		assert_eq!(
			faelle[2],
			("mehr_mindermengenliste", &[Netzbetreiber, Lieferant][..])
		);
		assert_eq!(
			faelle[3],
			("clearingliste", &[Netzbetreiber, Lieferant][..])
		);
		assert_eq!(
			faelle[4],
			(
				"bkv_anmeldung_bilanzkreis",
				&[Bilanzkreisverantwortlicher, Bilanzkoordinator][..]
			)
		);
		assert_eq!(
			faelle[5],
			(
				"biko_bestaetigung",
				&[Bilanzkoordinator, Bilanzkreisverantwortlicher][..]
			)
		);
		assert_eq!(
			faelle[6],
			(
				"anb_uebermittlung_an_biko",
				&[Anschlussnetzbetreiber, Bilanzkoordinator][..]
			)
		);
		assert_eq!(
			faelle[7],
			(
				"anfnb_anforderung",
				&[AnfordernderNetzbetreiber, Anschlussnetzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[8],
			(
				"netzbetreiber_uebertragungsnetz_aggregate",
				&[Netzbetreiber, Uebertragungsnetzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[9],
			(
				"biko_saldenbericht",
				&[Bilanzkoordinator, Uebertragungsnetzbetreiber][..]
			)
		);
	}
}

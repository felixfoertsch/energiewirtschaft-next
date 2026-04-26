use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenPfad = &'static [MarktRolle];

pub const ANMELDUNG_STEUERBARE_VERBRAUCHSEINRICHTUNG: RollenPfad =
	&[Lieferant, Netzbetreiber];
pub const MODUL_2_DATEN_AN_NETZBETREIBER: RollenPfad =
	&[Lieferant, Netzbetreiber];
pub const INFORMATION_AN_LIEFERANT: RollenPfad = &[Netzbetreiber, Lieferant];
pub const STEUERUNGSBEFEHL: RollenPfad = &[Netzbetreiber, Messstellenbetreiber];
pub const STEUERUNGSBEFEHL_GMSB: RollenPfad =
	&[Netzbetreiber, GrundzustaendigerMessstellenbetreiber];
pub const STEUERINFORMATIONS_QUITTUNG: RollenPfad =
	&[Messstellenbetreiber, Netzbetreiber];
pub const STEUERINFORMATIONS_QUITTUNG_GMSB: RollenPfad =
	&[GrundzustaendigerMessstellenbetreiber, Netzbetreiber];
pub const QUITTUNG_NB_AN_MSB: RollenPfad = &[Netzbetreiber, Messstellenbetreiber];
pub const QUITTUNG_MSB_AN_NB: RollenPfad = &[Messstellenbetreiber, Netzbetreiber];
pub const QUITTUNG_NB_AN_LIEFERANT: RollenPfad = &[Netzbetreiber, Lieferant];
pub const QUITTUNG_LIEFERANT_AN_NB: RollenPfad = &[Lieferant, Netzbetreiber];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollenpfade_sind_vollstaendiger_14a_kanon() {
		let faelle: &[(&str, RollenPfad)] = &[
			(
				"anmeldung_steuerbare_verbrauchseinrichtung",
				ANMELDUNG_STEUERBARE_VERBRAUCHSEINRICHTUNG,
			),
			(
				"modul_2_daten_an_netzbetreiber",
				MODUL_2_DATEN_AN_NETZBETREIBER,
			),
			("information_an_lieferant", INFORMATION_AN_LIEFERANT),
			("steuerungsbefehl", STEUERUNGSBEFEHL),
			("steuerungsbefehl_gmsb", STEUERUNGSBEFEHL_GMSB),
			(
				"steuerinformations_quittung",
				STEUERINFORMATIONS_QUITTUNG,
			),
			(
				"steuerinformations_quittung_gmsb",
				STEUERINFORMATIONS_QUITTUNG_GMSB,
			),
			("quittung_nb_an_msb", QUITTUNG_NB_AN_MSB),
			("quittung_msb_an_nb", QUITTUNG_MSB_AN_NB),
			("quittung_nb_an_lieferant", QUITTUNG_NB_AN_LIEFERANT),
			("quittung_lieferant_an_nb", QUITTUNG_LIEFERANT_AN_NB),
		];

		assert_eq!(
			faelle[0],
			(
				"anmeldung_steuerbare_verbrauchseinrichtung",
				&[Lieferant, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[1],
			(
				"modul_2_daten_an_netzbetreiber",
				&[Lieferant, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[2],
			("information_an_lieferant", &[Netzbetreiber, Lieferant][..])
		);
		assert_eq!(
			faelle[3],
			(
				"steuerungsbefehl",
				&[Netzbetreiber, Messstellenbetreiber][..]
			)
		);
		assert_eq!(
			faelle[4],
			(
				"steuerungsbefehl_gmsb",
				&[Netzbetreiber, GrundzustaendigerMessstellenbetreiber][..]
			)
		);
		assert_eq!(
			faelle[5],
			(
				"steuerinformations_quittung",
				&[Messstellenbetreiber, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[6],
			(
				"steuerinformations_quittung_gmsb",
				&[GrundzustaendigerMessstellenbetreiber, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[7],
			(
				"quittung_nb_an_msb",
				&[Netzbetreiber, Messstellenbetreiber][..]
			)
		);
		assert_eq!(
			faelle[8],
			(
				"quittung_msb_an_nb",
				&[Messstellenbetreiber, Netzbetreiber][..]
			)
		);
		assert_eq!(
			faelle[9],
			("quittung_nb_an_lieferant", &[Netzbetreiber, Lieferant][..])
		);
		assert_eq!(
			faelle[10],
			("quittung_lieferant_an_nb", &[Lieferant, Netzbetreiber][..])
		);
	}
}

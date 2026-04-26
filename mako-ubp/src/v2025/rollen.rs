use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenTupel = (MarktRolle, MarktRolle);

pub const ANGEBOTSANFRAGE: &[RollenTupel] = &[
	(Lieferant, Messstellenbetreiber),
	(Netzbetreiber, Messstellenbetreiber),
	(Energieserviceanbieter, Messstellenbetreiber),
	(Lieferant, GrundzustaendigerMessstellenbetreiber),
	(Lieferant, WettbewerblicherMessstellenbetreiber),
];
pub const ANGEBOT: &[RollenTupel] = &[
	(Messstellenbetreiber, Lieferant),
	(Messstellenbetreiber, Netzbetreiber),
	(Messstellenbetreiber, Energieserviceanbieter),
	(GrundzustaendigerMessstellenbetreiber, Lieferant),
	(WettbewerblicherMessstellenbetreiber, Lieferant),
];
pub const BESTELLUNG: &[RollenTupel] = ANGEBOTSANFRAGE;
pub const BESTELLANTWORT: &[RollenTupel] = ANGEBOT;
pub const PREISBLATT: &[RollenTupel] = &[
	(Messstellenbetreiber, Lieferant),
	(Messstellenbetreiber, Netzbetreiber),
	(GrundzustaendigerMessstellenbetreiber, Lieferant),
	(GrundzustaendigerMessstellenbetreiber, Netzbetreiber),
	(WettbewerblicherMessstellenbetreiber, Lieferant),
	(WettbewerblicherMessstellenbetreiber, Netzbetreiber),
];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollen_tupel_sind_vollstaendiger_ubp_kanon() {
		assert_eq!(
			ANGEBOTSANFRAGE,
			&[
				(Lieferant, Messstellenbetreiber),
				(Netzbetreiber, Messstellenbetreiber),
				(Energieserviceanbieter, Messstellenbetreiber),
				(Lieferant, GrundzustaendigerMessstellenbetreiber),
				(Lieferant, WettbewerblicherMessstellenbetreiber),
			]
		);
		assert_eq!(
			ANGEBOT,
			&[
				(Messstellenbetreiber, Lieferant),
				(Messstellenbetreiber, Netzbetreiber),
				(Messstellenbetreiber, Energieserviceanbieter),
				(GrundzustaendigerMessstellenbetreiber, Lieferant),
				(WettbewerblicherMessstellenbetreiber, Lieferant),
			]
		);
		assert_eq!(BESTELLUNG, ANGEBOTSANFRAGE);
		assert_eq!(BESTELLANTWORT, ANGEBOT);
		assert_eq!(
			PREISBLATT,
			&[
				(Messstellenbetreiber, Lieferant),
				(Messstellenbetreiber, Netzbetreiber),
				(GrundzustaendigerMessstellenbetreiber, Lieferant),
				(GrundzustaendigerMessstellenbetreiber, Netzbetreiber),
				(WettbewerblicherMessstellenbetreiber, Lieferant),
				(WettbewerblicherMessstellenbetreiber, Netzbetreiber),
			]
		);
	}
}

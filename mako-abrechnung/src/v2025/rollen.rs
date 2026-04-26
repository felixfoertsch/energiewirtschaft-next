use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenTupel = (MarktRolle, MarktRolle);

pub const RECHNUNG_INVOIC: &[RollenTupel] = &[
	(Rechnungsersteller, Rechnungsempfaenger),
	(Messstellenbetreiber, Lieferant),
	(GrundzustaendigerMessstellenbetreiber, Lieferant),
	(WettbewerblicherMessstellenbetreiber, Lieferant),
	(Netzbetreiber, Lieferant),
	(Lieferant, Netzbetreiber),
	(Netzbetreiber, Bilanzkreisverantwortlicher),
	(Marktgebietsverantwortlicher, Kapazitaetsnutzer),
	(Fernleitungsnetzbetreiber, Transportkunde),
];
pub const ZAHLUNGSAVIS_REMADV: &[RollenTupel] = &[
	(Rechnungsempfaenger, Rechnungsersteller),
	(Lieferant, Messstellenbetreiber),
	(Lieferant, GrundzustaendigerMessstellenbetreiber),
	(Lieferant, WettbewerblicherMessstellenbetreiber),
	(Lieferant, Netzbetreiber),
	(Netzbetreiber, Lieferant),
	(Bilanzkreisverantwortlicher, Netzbetreiber),
	(Kapazitaetsnutzer, Marktgebietsverantwortlicher),
	(Transportkunde, Fernleitungsnetzbetreiber),
];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollen_tupel_sind_vollstaendiger_abrechnung_kanon() {
		assert_eq!(
			RECHNUNG_INVOIC,
			&[
				(Rechnungsersteller, Rechnungsempfaenger),
				(Messstellenbetreiber, Lieferant),
				(GrundzustaendigerMessstellenbetreiber, Lieferant),
				(WettbewerblicherMessstellenbetreiber, Lieferant),
				(Netzbetreiber, Lieferant),
				(Lieferant, Netzbetreiber),
				(Netzbetreiber, Bilanzkreisverantwortlicher),
				(Marktgebietsverantwortlicher, Kapazitaetsnutzer),
				(Fernleitungsnetzbetreiber, Transportkunde),
			]
		);
		assert_eq!(
			ZAHLUNGSAVIS_REMADV,
			&[
				(Rechnungsempfaenger, Rechnungsersteller),
				(Lieferant, Messstellenbetreiber),
				(Lieferant, GrundzustaendigerMessstellenbetreiber),
				(Lieferant, WettbewerblicherMessstellenbetreiber),
				(Lieferant, Netzbetreiber),
				(Netzbetreiber, Lieferant),
				(Bilanzkreisverantwortlicher, Netzbetreiber),
				(Kapazitaetsnutzer, Marktgebietsverantwortlicher),
				(Transportkunde, Fernleitungsnetzbetreiber),
			]
		);
	}
}

use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenPfad = &'static [MarktRolle];

pub const KAPAZITAETSBUCHUNG_TK_MGV: RollenPfad = &[Transportkunde, Marktgebietsverantwortlicher];
pub const KAPAZITAETSBUCHUNG_KN_FNB: RollenPfad = &[Kapazitaetsnutzer, Fernleitungsnetzbetreiber];
pub const KAPAZITAETSABRECHNUNG_MGV_KN: RollenPfad =
	&[Marktgebietsverantwortlicher, Kapazitaetsnutzer];
pub const KAPAZITAETSABRECHNUNG_FNB_TK: RollenPfad = &[Fernleitungsnetzbetreiber, Transportkunde];
pub const SPEICHERZUGANG: RollenPfad = &[Transportkunde, Speicherstellenbetreiber];
pub const AUSSPEISEPUNKT: RollenPfad = &[Netzbetreiber, Fernleitungsnetzbetreiber];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollenpfade_sind_vollstaendiger_kov_kanon() {
		assert_eq!(
			KAPAZITAETSBUCHUNG_TK_MGV,
			&[Transportkunde, Marktgebietsverantwortlicher]
		);
		assert_eq!(
			KAPAZITAETSBUCHUNG_KN_FNB,
			&[Kapazitaetsnutzer, Fernleitungsnetzbetreiber]
		);
		assert_eq!(
			KAPAZITAETSABRECHNUNG_MGV_KN,
			&[Marktgebietsverantwortlicher, Kapazitaetsnutzer]
		);
		assert_eq!(
			KAPAZITAETSABRECHNUNG_FNB_TK,
			&[Fernleitungsnetzbetreiber, Transportkunde]
		);
		assert_eq!(SPEICHERZUGANG, &[Transportkunde, Speicherstellenbetreiber]);
		assert_eq!(AUSSPEISEPUNKT, &[Netzbetreiber, Fernleitungsnetzbetreiber]);
	}
}

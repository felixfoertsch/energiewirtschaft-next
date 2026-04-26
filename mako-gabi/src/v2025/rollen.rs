use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub type RollenPfad = &'static [MarktRolle];

pub const NOMINIERUNG: RollenPfad = &[Bilanzkreisverantwortlicher, Marktgebietsverantwortlicher];
pub const ALLOKATION: RollenPfad = &[Netzbetreiber, Marktgebietsverantwortlicher];
pub const TAGESWERTE: RollenPfad = &[Netzbetreiber, Marktgebietsverantwortlicher];
pub const KORREKTUR: RollenPfad = &[Marktgebietsverantwortlicher, Netzbetreiber];
pub const BILANZKREISZUORDNUNG_GAS: RollenPfad = &[
	Lieferant,
	Netzbetreiber,
	Bilanzkreisverantwortlicher,
	Marktgebietsverantwortlicher,
];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rollenpfade_sind_vollstaendiger_gabi_kanon() {
		assert_eq!(
			NOMINIERUNG,
			&[Bilanzkreisverantwortlicher, Marktgebietsverantwortlicher]
		);
		assert_eq!(ALLOKATION, &[Netzbetreiber, Marktgebietsverantwortlicher]);
		assert_eq!(TAGESWERTE, &[Netzbetreiber, Marktgebietsverantwortlicher]);
		assert_eq!(KORREKTUR, &[Marktgebietsverantwortlicher, Netzbetreiber]);
		assert_eq!(
			BILANZKREISZUORDNUNG_GAS,
			&[
				Lieferant,
				Netzbetreiber,
				Bilanzkreisverantwortlicher,
				Marktgebietsverantwortlicher
			]
		);
	}
}

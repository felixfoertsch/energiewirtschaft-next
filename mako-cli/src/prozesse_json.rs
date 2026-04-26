/// Print the full process catalog as JSON to stdout.
///
/// The test UI fetches this once at startup so the catalog stays in
/// lock-step with the engine's actual process implementations.
pub fn run() {
	print!("{}", mako_katalog::katalog_als_json());
}

#[cfg(test)]
mod coverage_tests {
	use crate::init::ROLLEN;
	use mako_katalog::alle_prozesse;
	use std::collections::HashSet;

	/// Every market role that `mako-cli init` creates a directory for must
	/// participate in at least one process. If this fails, a role was added
	/// without a corresponding process surface — the test UI would render
	/// "keine Prozesse" for that role, which is what this whole catalog
	/// refactor exists to prevent.
	///
	/// Currently ignored: the engine still misses reducers for several roles
	/// the spec requires (NetzbetreiberWechsel, MDL-Messdienstleistung,
	/// Direktvermarktung-MPES-Pfad, AGG-VPP, LPB-Ladepunkt, RB-HKNR-
	/// Herkunftsnachweise, ENB/ANBG-Gas-Netzknoten). Run with `cargo test
	/// -- --ignored` to see the live gap list. Removing #[ignore] is the
	/// release gate when those crates land.
	#[test]
	#[ignore = "engine implementation gap — see prozesse_json::coverage_tests doc comment"]
	fn jede_rolle_hat_mindestens_einen_prozess() {
		let prozesse = alle_prozesse();
		let beteiligte: HashSet<String> = prozesse
			.iter()
			.flat_map(|p| {
				p.schritte
					.iter()
					.flat_map(|s| [s.absender.clone(), s.empfaenger.clone()])
			})
			.collect();

		let waisen: Vec<&str> = ROLLEN
			.iter()
			.copied()
			.filter(|slug| !beteiligte.contains(*slug))
			.collect();

		assert!(
			waisen.is_empty(),
			"Rollen ohne Prozess: {waisen:?}\n\
			 Diese Rollen brauchen einen Eintrag im katalog.rs einer Prozess-Crate, \
			 sonst zeigt die Test-UI 'keine Prozesse'."
		);
	}
}

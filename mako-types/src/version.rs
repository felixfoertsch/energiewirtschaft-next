use serde::{Deserialize, Serialize};

/// All MaKo format versions / epochs.
/// Each process crate has a module per version (e.g. `gpke::v2025`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MakoVersion {
	/// MaKo 2017: MaLo/MeLo model, MSB as new role
	V2017,
	/// MaKo 2020: New GPKE, WiM, MaBiS, MPES
	V2020,
	/// MaKo 2022: Extended network access processes
	V2022,
	/// FV2504/LFW24: 24h supplier switch, API web services
	V2025,
}

impl MakoVersion {
	pub fn gueltig_ab(&self) -> &'static str {
		match self {
			Self::V2017 => "2017-10-01",
			Self::V2020 => "2020-02-01",
			Self::V2022 => "2023-10-01",
			Self::V2025 => "2025-06-06",
		}
	}
}

/// Trait for version-aware dispatching.
/// Each process crate can implement this to route to the correct versioned reducer.
pub trait VersionDispatcher {
	type State;
	type Event;
	type Output;
	type Error;

	fn dispatch(
		version: MakoVersion,
		state: Self::State,
		event: Self::Event,
	) -> Result<Self::Output, Self::Error>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn versions_are_ordered() {
		assert!(MakoVersion::V2017 < MakoVersion::V2020);
		assert!(MakoVersion::V2020 < MakoVersion::V2022);
		assert!(MakoVersion::V2022 < MakoVersion::V2025);
	}

	#[test]
	fn gueltig_ab_dates() {
		assert_eq!(MakoVersion::V2025.gueltig_ab(), "2025-06-06");
	}
}

pub mod v2017;
pub mod v2020;
pub mod v2022;
pub mod v2025;

#[cfg(test)]
mod dispatch_tests {
	#[test]
	fn can_use_different_version_modules() {
		// v2025 and v2022 are independent modules with independent types
		let _v2025_idle = crate::v2025::lfw::LfwState::Idle;
		let _v2022_idle = crate::v2022::lfw::LfwState::Idle;
		let _v2020_idle = crate::v2020::lfw::LfwState::Idle;
		let _v2017_idle = crate::v2017::lfw::LfwState::Idle;
	}
}

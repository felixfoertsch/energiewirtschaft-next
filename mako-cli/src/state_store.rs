use std::collections::HashMap;
use std::path::Path;

pub type StateMap = HashMap<String, serde_json::Value>;

pub fn load_state(rolle_dir: &Path) -> StateMap {
	let path = rolle_dir.join("state.json");
	if path.exists() {
		let content = std::fs::read_to_string(&path).unwrap_or_else(|_| "{}".into());
		serde_json::from_str(&content).unwrap_or_default()
	} else {
		HashMap::new()
	}
}

pub fn save_state(rolle_dir: &Path, state: &StateMap) {
	let path = rolle_dir.join("state.json");
	let content = serde_json::to_string_pretty(state).expect("serialize state");
	std::fs::write(path, content).expect("write state.json");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn roundtrip_state() {
		let dir = std::env::temp_dir().join("mako-cli-test-state");
		std::fs::create_dir_all(&dir).ok();
		let mut states = StateMap::new();
		states.insert("gpke_lfw/123".into(), serde_json::json!({"state": "Idle"}));
		save_state(&dir, &states);
		let loaded = load_state(&dir);
		assert_eq!(
			loaded.get("gpke_lfw/123").unwrap(),
			&serde_json::json!({"state": "Idle"})
		);
		std::fs::remove_dir_all(&dir).ok();
	}

	#[test]
	fn load_missing_returns_empty() {
		let dir = std::env::temp_dir().join("mako-cli-test-state-missing");
		// ensure directory does not exist
		std::fs::remove_dir_all(&dir).ok();
		std::fs::create_dir_all(&dir).ok();
		let loaded = load_state(&dir);
		assert!(loaded.is_empty());
		std::fs::remove_dir_all(&dir).ok();
	}
}

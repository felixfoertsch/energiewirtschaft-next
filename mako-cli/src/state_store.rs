use std::collections::HashMap;
use std::path::Path;

pub type StateMap = HashMap<String, serde_json::Value>;

pub fn load_state(rolle_dir: &Path) -> Result<StateMap, Box<dyn std::error::Error>> {
	let path = rolle_dir.join("state.json");
	if !path.exists() {
		return Ok(HashMap::new());
	}
	let content = std::fs::read_to_string(&path)
		.map_err(|e| format!("state.json nicht lesbar ({}): {e}", path.display()))?;
	let map: StateMap = serde_json::from_str(&content)
		.map_err(|e| format!("state.json ist kein gültiges JSON ({}): {e}", path.display()))?;
	Ok(map)
}

pub fn save_state(rolle_dir: &Path, state: &StateMap) -> Result<(), Box<dyn std::error::Error>> {
	let path = rolle_dir.join("state.json");
	let content = serde_json::to_string_pretty(state)?;
	std::fs::write(&path, content)
		.map_err(|e| format!("state.json nicht schreibbar ({}): {e}", path.display()))?;
	Ok(())
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
		save_state(&dir, &states).unwrap();
		let loaded = load_state(&dir).unwrap();
		assert_eq!(
			loaded.get("gpke_lfw/123").unwrap(),
			&serde_json::json!({"state": "Idle"})
		);
		std::fs::remove_dir_all(&dir).ok();
	}

	#[test]
	fn load_missing_returns_empty() {
		let dir = std::env::temp_dir().join("mako-cli-test-state-missing");
		std::fs::remove_dir_all(&dir).ok();
		std::fs::create_dir_all(&dir).ok();
		let loaded = load_state(&dir).unwrap();
		assert!(loaded.is_empty());
		std::fs::remove_dir_all(&dir).ok();
	}

	#[test]
	fn load_corrupt_state_errors() {
		let dir = std::env::temp_dir().join("mako-cli-test-state-corrupt");
		std::fs::remove_dir_all(&dir).ok();
		std::fs::create_dir_all(&dir).ok();
		std::fs::write(dir.join("state.json"), "not json").unwrap();
		let result = load_state(&dir);
		assert!(result.is_err(), "corrupt state.json must surface an error");
		std::fs::remove_dir_all(&dir).ok();
	}
}

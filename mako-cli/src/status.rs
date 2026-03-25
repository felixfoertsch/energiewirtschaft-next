pub fn run(markt: &str) -> Result<(), Box<dyn std::error::Error>> {
	let markt_path = std::path::Path::new(markt);

	println!("Markt-Status ({}):", markt);
	println!("{:<28} {:>8} {:>8}  Prozesse", "Rolle", "Inbox", "Outbox");
	println!("{}", "-".repeat(70));

	let mut entries: Vec<_> = std::fs::read_dir(markt_path)?
		.filter_map(|e| e.ok())
		.filter(|e| {
			e.path().is_dir() && e.file_name() != "log"
		})
		.collect();

	// Sort alphabetically for stable output
	entries.sort_by_key(|e| e.file_name());

	for entry in entries {
		let path = entry.path();
		let name = entry.file_name().to_string_lossy().to_string();
		let inbox_count = count_files(&path.join("inbox"));
		let outbox_count = count_files(&path.join("outbox"));

		// Read state.json for active processes
		let states = crate::state_store::load_state(&path);
		let mut proz: Vec<String> = states.keys().cloned().collect();
		proz.sort();
		let proz_str = if proz.is_empty() { "-".to_string() } else { proz.join(", ") };

		println!("{:<28} {:>8} {:>8}  {}", name, inbox_count, outbox_count, proz_str);
	}

	Ok(())
}

pub fn count_files(dir: &std::path::Path) -> usize {
	std::fs::read_dir(dir)
		.map(|entries| {
			entries
				.filter_map(|e| e.ok())
				.filter(|e| e.path().is_file())
				.count()
		})
		.unwrap_or(0)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn setup_markt() -> (tempfile::TempDir, std::path::PathBuf) {
		let tmp = tempfile::tempdir().expect("temp dir");
		let markt = tmp.path().join("markt");
		crate::init::run(markt.to_str().unwrap());
		(tmp, markt)
	}

	#[test]
	fn test_status_counts_files() {
		let (_tmp, markt) = setup_markt();

		// Place 2 files in netzbetreiber/inbox
		let inbox = markt.join("netzbetreiber").join("inbox");
		std::fs::create_dir_all(&inbox).unwrap();
		std::fs::write(inbox.join("a.json"), "{}").unwrap();
		std::fs::write(inbox.join("b.json"), "{}").unwrap();

		assert_eq!(count_files(&inbox), 2);

		// Place 1 file in netzbetreiber/outbox
		let outbox = markt.join("netzbetreiber").join("outbox");
		std::fs::create_dir_all(&outbox).unwrap();
		std::fs::write(outbox.join("c.json"), "{}").unwrap();

		assert_eq!(count_files(&outbox), 1);
	}

	#[test]
	fn test_status_empty_dir_returns_zero() {
		let tmp = tempfile::tempdir().unwrap();
		assert_eq!(count_files(tmp.path()), 0);
		// Non-existent directory also returns 0
		assert_eq!(count_files(&tmp.path().join("does_not_exist")), 0);
	}

	#[test]
	fn test_status_run_succeeds() {
		let (_tmp, markt) = setup_markt();
		let result = run(markt.to_str().unwrap());
		assert!(result.is_ok(), "status run fehlgeschlagen: {result:?}");
	}
}

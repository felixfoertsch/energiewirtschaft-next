use mako_verify::bericht::{BatchErgebnis, Urteil, VerifikationsErgebnis};

fn ergebnis_mit_urteil(urteil: Urteil) -> VerifikationsErgebnis {
	VerifikationsErgebnis {
		datei: "test.edi".to_string(),
		nachrichtentyp: "UTILMD".to_string(),
		pruefidentifikator: Some("44001".to_string()),
		ahb: None,
		ebd: None,
		interop: None,
		gesamt_urteil: urteil,
	}
}

#[test]
fn zusammenfassung_formatierung() {
	let batch = BatchErgebnis {
		gesamt: 5,
		bestanden: 3,
		fehlgeschlagen: 1,
		nicht_pruefbar: 1,
		ergebnisse: Vec::new(),
	};

	assert_eq!(
		batch.zusammenfassung(),
		"5 geprüft, 3 bestanden, 1 fehlgeschlagen, 1 nicht prüfbar"
	);
}

#[test]
fn aus_ergebnissen_zaehlt_korrekt() {
	let ergebnisse = vec![
		ergebnis_mit_urteil(Urteil::Bestanden),
		ergebnis_mit_urteil(Urteil::Bestanden),
		ergebnis_mit_urteil(Urteil::Fehlgeschlagen),
		ergebnis_mit_urteil(Urteil::NichtPruefbar),
		ergebnis_mit_urteil(Urteil::Bestanden),
	];

	let batch = BatchErgebnis::aus_ergebnissen(ergebnisse);

	assert_eq!(batch.gesamt, 5);
	assert_eq!(batch.bestanden, 3);
	assert_eq!(batch.fehlgeschlagen, 1);
	assert_eq!(batch.nicht_pruefbar, 1);
	assert_eq!(
		batch.zusammenfassung(),
		"5 geprüft, 3 bestanden, 1 fehlgeschlagen, 1 nicht prüfbar"
	);
}

#[test]
fn leere_batch_ergebnisse() {
	let batch = BatchErgebnis::aus_ergebnissen(Vec::new());

	assert_eq!(batch.gesamt, 0);
	assert_eq!(batch.bestanden, 0);
	assert_eq!(batch.fehlgeschlagen, 0);
	assert_eq!(batch.nicht_pruefbar, 0);
	assert_eq!(
		batch.zusammenfassung(),
		"0 geprüft, 0 bestanden, 0 fehlgeschlagen, 0 nicht prüfbar"
	);
}

#[test]
fn urteil_eq() {
	// Correction A: Urteil must derive Eq
	assert_eq!(Urteil::Bestanden, Urteil::Bestanden);
	assert_ne!(Urteil::Bestanden, Urteil::Fehlgeschlagen);
	assert_ne!(Urteil::Fehlgeschlagen, Urteil::NichtPruefbar);
}

#[test]
fn verifikations_ergebnis_serialisiert() {
	let ergebnis = ergebnis_mit_urteil(Urteil::Bestanden);
	let json = serde_json::to_string(&ergebnis).expect("should serialize");
	assert!(json.contains("\"Bestanden\""));
	assert!(json.contains("\"UTILMD\""));
}

#[test]
fn batch_ergebnis_serialisiert() {
	let batch = BatchErgebnis::aus_ergebnissen(vec![
		ergebnis_mit_urteil(Urteil::Bestanden),
		ergebnis_mit_urteil(Urteil::Fehlgeschlagen),
	]);
	let json = serde_json::to_string(&batch).expect("should serialize");
	assert!(json.contains("\"gesamt\":2"));
}

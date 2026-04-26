use chrono::{NaiveDate, NaiveDateTime};

use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_melo, test_mp_id};

// ---------------------------------------------------------------------------
// 1. Anmeldung (E01 / PID 44001) — LFN -> NB
// ---------------------------------------------------------------------------

pub fn anmeldung_lfw_edi() -> String {
	let sender = test_mp_id(0);
	let empfaenger = test_mp_id(1);
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260701:102'\
		 RFF+Z13:44001'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn anmeldung_lfw_erwartet() -> Nachricht {
	let sender = test_mp_id(0);
	let empfaenger = test_mp_id(1);

	Nachricht {
		absender: sender.clone(),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: Some(PruefIdentifikator::AnmeldungNn),
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: sender,
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 2. Bestaetigung (E01 / PID 44002) — NB -> LFN
// ---------------------------------------------------------------------------

pub fn bestaetigung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LFN
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260701:102'\
		 RFF+Z13:44002'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn bestaetigung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lfn = test_mp_id(0);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lfn.clone(),
		empfaenger_rolle: MarktRolle::LieferantNeu,
		pruef_id: Some(PruefIdentifikator::AnmeldungBestaetigung),
		payload: NachrichtenPayload::UtilmdBestaetigung(UtilmdBestaetigung {
			malo_id: test_malo(0),
			bestaetigt_fuer: lfn,
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 3. Abmeldung (E02 / PID 44004) — NB -> LFA
// ---------------------------------------------------------------------------

pub fn abmeldung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(2); // LFA
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E02+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260630:102'\
		 RFF+Z13:44004'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn abmeldung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lfa = test_mp_id(2);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lfa.clone(),
		empfaenger_rolle: MarktRolle::LieferantAlt,
		pruef_id: Some(PruefIdentifikator::AbmeldungNn),
		payload: NachrichtenPayload::UtilmdAbmeldung(UtilmdAbmeldung {
			malo_id: test_malo(0),
			lieferant_alt: lfa,
			lieferende: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 4. Ablehnung (E01 / PID 44003) — LFA -> NB
// ---------------------------------------------------------------------------

pub fn ablehnung_edi() -> String {
	let sender = test_mp_id(2); // LFA
	let empfaenger = test_mp_id(1); // NB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 STS+FRIST'\
		 RFF+Z13:44003'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn ablehnung_erwartet() -> Nachricht {
	let lfa = test_mp_id(2);
	let nb = test_mp_id(1);

	Nachricht {
		absender: lfa,
		absender_rolle: MarktRolle::LieferantAlt,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: Some(PruefIdentifikator::AnmeldungAblehnung),
		payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung {
			malo_id: test_malo(0),
			grund: AblehnungsGrund::Fristverletzung,
		}),
	}
}

// ---------------------------------------------------------------------------
// 5. Zuordnung (E06 / PID 44005) — NB -> LFN
// ---------------------------------------------------------------------------

pub fn zuordnung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LFN
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E06+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260701:102'\
		 RFF+Z13:44005'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn zuordnung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lfn = test_mp_id(0);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lfn.clone(),
		empfaenger_rolle: MarktRolle::LieferantNeu,
		pruef_id: Some(PruefIdentifikator::AbmeldungBestaetigung),
		payload: NachrichtenPayload::UtilmdZuordnung(UtilmdZuordnung {
			malo_id: test_malo(0),
			zugeordnet_an: lfn,
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 6. LieferendeAbmeldung (E02 / PID 44006) — LF -> NB
// ---------------------------------------------------------------------------

pub fn lieferende_abmeldung_edi() -> String {
	let sender = test_mp_id(3); // LF
	let empfaenger = test_mp_id(1); // NB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E02+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260930:102'\
		 RFF+Z13:44006'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn lieferende_abmeldung_erwartet() -> Nachricht {
	let lf = test_mp_id(3);
	let nb = test_mp_id(1);

	Nachricht {
		absender: lf.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: Some(PruefIdentifikator::AbmeldungAblehnung),
		payload: NachrichtenPayload::UtilmdLieferendeAbmeldung(UtilmdLieferendeAbmeldung {
			malo_id: test_malo(0),
			lieferant: lf,
			lieferende: NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 7. LieferendeBestaetigung (E01 / no PID) — NB -> LF
// ---------------------------------------------------------------------------

pub fn lieferende_bestaetigung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // LF
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260930:102'\
		 UNT+8+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn lieferende_bestaetigung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(3);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdLieferendeBestaetigung(UtilmdLieferendeBestaetigung {
			malo_id: test_malo(0),
			lieferende: NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 8. Stammdatenaenderung (E03 / PID 44112) — NB -> LF
// ---------------------------------------------------------------------------

pub fn stammdatenaenderung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // LF
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E03+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 RFF+Z13:44112'\
		 CCI+Spannungsebene'\
		 CAV+Niederspannung'\
		 CCI+Netzgebiet'\
		 CAV+Berlin'\
		 UNT+12+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn stammdatenaenderung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(3);

	Nachricht {
		absender: nb.clone(),
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: Some(PruefIdentifikator::Stammdatenaenderung),
		payload: NachrichtenPayload::UtilmdStammdatenaenderung(UtilmdStammdatenaenderung {
			malo_id: test_malo(0),
			initiator: nb,
			aenderungen: vec![
				Stammdatenfeld {
					feld: "Spannungsebene".to_string(),
					alter_wert: None,
					neuer_wert: "Niederspannung".to_string(),
				},
				Stammdatenfeld {
					feld: "Netzgebiet".to_string(),
					alter_wert: None,
					neuer_wert: "Berlin".to_string(),
				},
			],
		}),
	}
}

// ---------------------------------------------------------------------------
// 9. Zuordnungsliste (E06 / no PID, multiple IDE) — NB -> LF
// ---------------------------------------------------------------------------

pub fn zuordnungsliste_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // LF
	let malo0 = test_malo(0);
	let malo1 = test_malo(1);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E06+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo0}'\
		 DTM+92:20260701:102'\
		 IDE+24+{malo1}'\
		 DTM+92:20260801:102'\
		 UNT+10+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo0 = malo0.as_str(),
		malo1 = malo1.as_str(),
	)
}

pub fn zuordnungsliste_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(3);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf.clone(),
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdZuordnungsliste(UtilmdZuordnungsliste {
			eintraege: vec![
				ZuordnungsEintrag {
					malo_id: test_malo(0),
					zugeordnet_an: lf.clone(),
					gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
				},
				ZuordnungsEintrag {
					malo_id: test_malo(1),
					zugeordnet_an: lf,
					gueltig_ab: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
				},
			],
		}),
	}
}

// ---------------------------------------------------------------------------
// 10. Geschaeftsdatenanfrage (E09 / no PID) — LF -> NB
// ---------------------------------------------------------------------------

pub fn geschaeftsdatenanfrage_edi() -> String {
	let sender = test_mp_id(3); // LF
	let empfaenger = test_mp_id(1); // NB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E09+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 UNT+7+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn geschaeftsdatenanfrage_erwartet() -> Nachricht {
	let lf = test_mp_id(3);
	let nb = test_mp_id(1);

	Nachricht {
		absender: lf.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdGeschaeftsdatenanfrage(UtilmdGeschaeftsdatenanfrage {
			malo_id: test_malo(0),
			anfragender: lf,
		}),
	}
}

// ---------------------------------------------------------------------------
// 11. Geschaeftsdatenantwort (E09 resp / no PID, with CCI) — NB -> LF
// ---------------------------------------------------------------------------

pub fn geschaeftsdatenantwort_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // LF
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E09+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 CCI+Spannungsebene'\
		 CAV+Niederspannung'\
		 CCI+Netzgebiet'\
		 CAV+Berlin'\
		 UNT+11+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn geschaeftsdatenantwort_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(3);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdGeschaeftsdatenantwort(UtilmdGeschaeftsdatenantwort {
			malo_id: test_malo(0),
			stammdaten: vec![
				Stammdatenfeld {
					feld: "Spannungsebene".to_string(),
					alter_wert: None,
					neuer_wert: "Niederspannung".to_string(),
				},
				Stammdatenfeld {
					feld: "Netzgebiet".to_string(),
					alter_wert: None,
					neuer_wert: "Berlin".to_string(),
				},
			],
		}),
	}
}

// ===========================================================================
// WiM Variants
// ===========================================================================

// ---------------------------------------------------------------------------
// 14. MSB-Wechsel Anmeldung (E03, MeLo-ID, no PID) — MSB -> NB
// ---------------------------------------------------------------------------

pub fn msb_wechsel_anmeldung_edi() -> String {
	let sender = test_mp_id(3); // MSB
	let empfaenger = test_mp_id(1); // NB
	let melo = test_melo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E03+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{melo}'\
		 DTM+92:20260801:102'\
		 UNT+8+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		melo = melo.as_str(),
	)
}

pub fn msb_wechsel_anmeldung_erwartet() -> Nachricht {
	let msb = test_mp_id(3);
	let nb = test_mp_id(1);

	Nachricht {
		absender: msb.clone(),
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdMsbWechselAnmeldung(UtilmdMsbWechselAnmeldung {
			melo_id: test_melo(0),
			msb_neu: msb,
			wechseldatum: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 15. Geraetewechsel (E03, MeLo-ID, CCI+Z30 pairs) — MSB -> NB
// ---------------------------------------------------------------------------

pub fn geraetewechsel_edi() -> String {
	let sender = test_mp_id(3); // MSB
	let empfaenger = test_mp_id(1); // NB
	let melo = test_melo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E03+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{melo}'\
		 DTM+92:20260801:102'\
		 CCI+Z30'\
		 CAV+ALT-1234'\
		 CCI+Z30'\
		 CAV+NEU-5678'\
		 UNT+12+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		melo = melo.as_str(),
	)
}

pub fn geraetewechsel_erwartet() -> Nachricht {
	let msb = test_mp_id(3);
	let nb = test_mp_id(1);

	Nachricht {
		absender: msb,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdGeraetewechsel(UtilmdGeraetewechsel {
			melo_id: test_melo(0),
			alte_geraete_nr: "ALT-1234".to_string(),
			neue_geraete_nr: "NEU-5678".to_string(),
			wechseldatum: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
		}),
	}
}

// ===========================================================================
// MaBiS Variants
// ===========================================================================

// ---------------------------------------------------------------------------
// 16. Bilanzkreiszuordnung (E01, RFF+Z06) — LF -> NB
// ---------------------------------------------------------------------------

pub fn bilanzkreiszuordnung_edi() -> String {
	let sender = test_mp_id(0); // LF
	let empfaenger = test_mp_id(1); // NB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260701:102'\
		 RFF+Z06:11XDE-BKTEST-X'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn bilanzkreiszuordnung_erwartet() -> Nachricht {
	let lf = test_mp_id(0);
	let nb = test_mp_id(1);

	Nachricht {
		absender: lf,
		absender_rolle: MarktRolle::Lieferant,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdBilanzkreiszuordnung(UtilmdBilanzkreiszuordnung {
			malo_id: test_malo(0),
			bilanzkreis: "11XDE-BKTEST-X".to_string(),
			gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 17. Clearingliste (E06, CCI+CLEARING entries) — NB -> LF
// ---------------------------------------------------------------------------

pub fn clearingliste_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LF
	let malo0 = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E06+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 CCI+CLEARING:{malo0}:Spannungsebene'\
		 CAV+Niederspannung:Mittelspannung'\
		 UNT+8+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo0 = malo0.as_str(),
	)
}

pub fn clearingliste_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdClearingliste(UtilmdClearingliste {
			eintraege: vec![ClearingEintrag {
				malo_id: test_malo(0),
				feld: "Spannungsebene".to_string(),
				nb_wert: "Niederspannung".to_string(),
				lf_wert: Some("Mittelspannung".to_string()),
			}],
		}),
	}
}

// ===========================================================================
// MPES Variants
// ===========================================================================

// ---------------------------------------------------------------------------
// MPES 1. AnmeldungErzeugung (E01, QTY+220, CCI eeg) — BetreiberErzeugungsanlage -> NB
// ---------------------------------------------------------------------------

pub fn anmeldung_erzeugung_edi() -> String {
	let sender = test_mp_id(5); // BetreiberErzeugungsanlage
	let empfaenger = test_mp_id(1); // NB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 LOC+172+{malo}'\
		 CCI+Z30::true'\
		 QTY+220:9.9:kW'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn anmeldung_erzeugung_erwartet() -> Nachricht {
	let bea = test_mp_id(5);
	let nb = test_mp_id(1);

	Nachricht {
		absender: bea.clone(),
		absender_rolle: MarktRolle::BetreiberErzeugungsanlage,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdAnmeldungErzeugung(UtilmdAnmeldungErzeugung {
			malo_id: test_malo(0),
			anlagenbetreiber: bea,
			eeg_anlage: true,
			installierte_leistung_kw: 9.9,
		}),
	}
}

// ===========================================================================
// §14a Variants
// ===========================================================================

// ---------------------------------------------------------------------------
// §14a 3. SteuerbareVerbrauchseinrichtung (E01, CCI geraetetyp) — NB -> MSB
// ---------------------------------------------------------------------------

pub fn steuerbare_verbrauchseinrichtung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // MSB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 LOC+172+{malo}'\
		 CCI+Z30::Wallbox'\
		 QTY+220:11:kW'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn steuerbare_verbrauchseinrichtung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let msb = test_mp_id(3);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: msb,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdSteuerbareVerbrauchseinrichtung(
			UtilmdSteuerbareVerbrauchseinrichtung {
				malo_id: test_malo(0),
				geraetetyp: SteuerbarerGeraetetyp::Wallbox,
				max_leistung_kw: 11.0,
			},
		),
	}
}

// ---------------------------------------------------------------------------
// §14a 4. ClsSteuersignal (E04, STS steuerung) — NB -> MSB
// ---------------------------------------------------------------------------

pub fn cls_steuersignal_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // MSB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E04+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 LOC+172+{malo}'\
		 STS+7++Z08'\
		 DTM+163:20260701140000:203'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn cls_steuersignal_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let msb = test_mp_id(3);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: msb,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::ClsSteuersignal(ClsSteuersignal {
			malo_id: test_malo(0),
			steuerung: Steuerungsart::Abschaltung,
			zeitpunkt: NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
				chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
			),
		}),
	}
}

// ===========================================================================
// Gas Variants (UTILMD)
// ===========================================================================

// ---------------------------------------------------------------------------
// Gas 9. Ausspeisepunkt (E01, two NAD+DP) — NB -> FNB
// ---------------------------------------------------------------------------

pub fn ausspeisepunkt_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(6); // FNB
	let malo = test_malo(0);
	let nb_dp = test_mp_id(1);
	let fnb_dp = test_mp_id(6);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 LOC+172+{malo}'\
		 NAD+DP+{nb_dp}::293'\
		 NAD+DP+{fnb_dp}::293'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
		nb_dp = nb_dp.as_str(),
		fnb_dp = fnb_dp.as_str(),
	)
}

pub fn ausspeisepunkt_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let fnb = test_mp_id(6);

	Nachricht {
		absender: nb.clone(),
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: fnb.clone(),
		empfaenger_rolle: MarktRolle::Fernleitungsnetzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdAusspeisepunkt(UtilmdAusspeisepunkt {
			malo_id: test_malo(0),
			nb,
			fnb,
		}),
	}
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use mako_codec::edifact::dispatch::{parse_nachricht, serialize_nachricht};

	use super::*;

	// --- 1. Anmeldung ---

	#[test]
	fn parse_anmeldung_lfw() {
		let edi = anmeldung_lfw_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		let erwartet = anmeldung_lfw_erwartet();
		assert_eq!(parsed, erwartet);
	}

	#[test]
	fn roundtrip_anmeldung_lfw() {
		let edi = anmeldung_lfw_edi();
		let parsed = parse_nachricht(&edi).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 2. Bestaetigung ---

	#[test]
	fn parse_bestaetigung() {
		let edi = bestaetigung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, bestaetigung_erwartet());
	}

	#[test]
	fn roundtrip_bestaetigung() {
		let parsed = parse_nachricht(&bestaetigung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 3. Abmeldung ---

	#[test]
	fn parse_abmeldung() {
		let edi = abmeldung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, abmeldung_erwartet());
	}

	#[test]
	fn roundtrip_abmeldung() {
		let parsed = parse_nachricht(&abmeldung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 4. Ablehnung ---

	#[test]
	fn parse_ablehnung() {
		let edi = ablehnung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, ablehnung_erwartet());
	}

	#[test]
	fn roundtrip_ablehnung() {
		let parsed = parse_nachricht(&ablehnung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 5. Zuordnung ---

	#[test]
	fn parse_zuordnung() {
		let edi = zuordnung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, zuordnung_erwartet());
	}

	#[test]
	fn roundtrip_zuordnung() {
		let parsed = parse_nachricht(&zuordnung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 6. LieferendeAbmeldung ---

	#[test]
	fn parse_lieferende_abmeldung() {
		let edi = lieferende_abmeldung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, lieferende_abmeldung_erwartet());
	}

	#[test]
	fn roundtrip_lieferende_abmeldung() {
		let parsed = parse_nachricht(&lieferende_abmeldung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 7. LieferendeBestaetigung ---

	#[test]
	fn parse_lieferende_bestaetigung() {
		let edi = lieferende_bestaetigung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, lieferende_bestaetigung_erwartet());
	}

	#[test]
	fn roundtrip_lieferende_bestaetigung() {
		let parsed = parse_nachricht(&lieferende_bestaetigung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 8. Stammdatenaenderung ---

	#[test]
	fn parse_stammdatenaenderung() {
		let edi = stammdatenaenderung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, stammdatenaenderung_erwartet());
	}

	#[test]
	fn roundtrip_stammdatenaenderung() {
		let parsed = parse_nachricht(&stammdatenaenderung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 9. Zuordnungsliste ---

	#[test]
	fn parse_zuordnungsliste() {
		let edi = zuordnungsliste_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, zuordnungsliste_erwartet());
	}

	#[test]
	fn roundtrip_zuordnungsliste() {
		let parsed = parse_nachricht(&zuordnungsliste_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 10. Geschaeftsdatenanfrage ---

	#[test]
	fn parse_geschaeftsdatenanfrage() {
		let edi = geschaeftsdatenanfrage_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, geschaeftsdatenanfrage_erwartet());
	}

	#[test]
	fn roundtrip_geschaeftsdatenanfrage() {
		let parsed = parse_nachricht(&geschaeftsdatenanfrage_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 11. Geschaeftsdatenantwort ---

	#[test]
	fn parse_geschaeftsdatenantwort() {
		let edi = geschaeftsdatenantwort_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, geschaeftsdatenantwort_erwartet());
	}

	#[test]
	fn roundtrip_geschaeftsdatenantwort() {
		let parsed = parse_nachricht(&geschaeftsdatenantwort_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 14. MSB-Wechsel Anmeldung ---

	#[test]
	fn parse_msb_wechsel_anmeldung() {
		let edi = msb_wechsel_anmeldung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, msb_wechsel_anmeldung_erwartet());
	}

	#[test]
	fn roundtrip_msb_wechsel_anmeldung() {
		let parsed = parse_nachricht(&msb_wechsel_anmeldung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 15. Geraetewechsel ---

	#[test]
	fn parse_geraetewechsel() {
		let edi = geraetewechsel_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, geraetewechsel_erwartet());
	}

	#[test]
	fn roundtrip_geraetewechsel() {
		let parsed = parse_nachricht(&geraetewechsel_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 16. Bilanzkreiszuordnung ---

	#[test]
	fn parse_bilanzkreiszuordnung() {
		let edi = bilanzkreiszuordnung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, bilanzkreiszuordnung_erwartet());
	}

	#[test]
	fn roundtrip_bilanzkreiszuordnung() {
		let parsed = parse_nachricht(&bilanzkreiszuordnung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 17. Clearingliste ---

	#[test]
	fn parse_clearingliste() {
		let edi = clearingliste_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, clearingliste_erwartet());
	}

	#[test]
	fn roundtrip_clearingliste() {
		let parsed = parse_nachricht(&clearingliste_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- MPES: AnmeldungErzeugung ---

	#[test]
	fn parse_anmeldung_erzeugung() {
		let edi = anmeldung_erzeugung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, anmeldung_erzeugung_erwartet());
	}

	#[test]
	fn roundtrip_anmeldung_erzeugung() {
		let parsed = parse_nachricht(&anmeldung_erzeugung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- §14a: SteuerbareVerbrauchseinrichtung ---

	#[test]
	fn parse_steuerbare_verbrauchseinrichtung() {
		let edi = steuerbare_verbrauchseinrichtung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, steuerbare_verbrauchseinrichtung_erwartet());
	}

	#[test]
	fn roundtrip_steuerbare_verbrauchseinrichtung() {
		let parsed = parse_nachricht(&steuerbare_verbrauchseinrichtung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- §14a: ClsSteuersignal ---

	#[test]
	fn parse_cls_steuersignal() {
		let edi = cls_steuersignal_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, cls_steuersignal_erwartet());
	}

	#[test]
	fn roundtrip_cls_steuersignal() {
		let parsed = parse_nachricht(&cls_steuersignal_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- Gas: Ausspeisepunkt ---

	#[test]
	fn parse_ausspeisepunkt() {
		let edi = ausspeisepunkt_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, ausspeisepunkt_erwartet());
	}

	#[test]
	fn roundtrip_ausspeisepunkt() {
		let parsed = parse_nachricht(&ausspeisepunkt_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}

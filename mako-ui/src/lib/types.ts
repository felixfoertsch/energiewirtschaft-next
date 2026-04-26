export interface Rolle {
	name: string;
	mp_id: string;
	verzeichnis: string;
}

export interface NachrichtMeta {
	datei: string;
	typ: string;
	absender: string;
	empfaenger: string;
	zeitpunkt: string;
	status: NachrichtenStatus;
}

export interface NachrichtenStatus {
	erstellt?: string;
	zugestellt?: string;
	contrl?: { ergebnis: "positiv" | "negativ"; zeitpunkt: string };
	aperak?: { ergebnis: "positiv" | "negativ"; zeitpunkt: string };
	verarbeitet?: string;
}

export interface ProzessSchritt {
	name: string;
	absender_rolle: string;
	empfaenger_rolle: string;
	nachrichtentyp: string;
	status: "done" | "current" | "pending";
}

export interface ProzessDefinition {
	name: string;
	kategorie: string;
	schritte: Omit<ProzessSchritt, "status">[];
}

export interface MarktStatus {
	rollen: RollenStatus[];
}

export interface RollenStatus {
	name: string;
	inbox_count: number;
	outbox_count: number;
	prozesse: ProzessInfo[];
}

export interface ProzessInfo {
	key: string;
	beschreibung: string;
	zustand: string;
}

// ---------------------------------------------------------------------------
// Verification types (matching mako-verify/src/bericht.rs)
// ---------------------------------------------------------------------------

export type Urteil = "Bestanden" | "Fehlgeschlagen" | "NichtPruefbar";

export interface AhbFeldErgebnis {
	segment_code: string | null;
	segment_group: string | null;
	data_element: string | null;
	name: string;
	ahb_ausdruck: string;
	unser_wert: string | null;
	erwarteter_wert: string | null;
	urteil: Urteil;
	details: string | null;
}

export interface AhbErgebnis {
	pruefidentifikator: string;
	nachrichtentyp: string;
	felder: AhbFeldErgebnis[];
	urteil: Urteil;
	zusammenfassung: string | null;
}

export interface EbdAusgang {
	ebd_code: string;
	schritt: string;
	beschreibung: string;
	antwortcode: string | null;
	notiz: string | null;
}

export interface EbdErgebnis {
	ebd_code: string;
	ebd_name: string;
	rolle: string;
	unser_ergebnis: EbdAusgang | null;
	gueltige_ausgaenge: EbdAusgang[];
	urteil: Urteil;
	details: string | null;
}

export interface InteropFeldVergleich {
	feld: string;
	unser_wert: string | null;
	drittanbieter_wert: string | null;
	stimmt_ueberein: boolean;
}

export interface InteropErgebnis {
	parse_ok_unser: boolean;
	parse_ok_drittanbieter: boolean;
	roundtrip_ok: boolean;
	feldvergleiche: InteropFeldVergleich[];
	urteil: Urteil;
}

export interface VerifikationsErgebnis {
	datei: string;
	nachrichtentyp: string;
	pruefidentifikator: string | null;
	ahb: AhbErgebnis | null;
	ebd: EbdErgebnis | null;
	interop: InteropErgebnis | null;
	gesamt_urteil: Urteil;
	/** Present when verification could not run (parse/IO failure). */
	fehler?: string;
}

export interface BatchErgebnis {
	gesamt: number;
	bestanden: number;
	fehlgeschlagen: number;
	nicht_pruefbar: number;
	ergebnisse: VerifikationsErgebnis[];
}

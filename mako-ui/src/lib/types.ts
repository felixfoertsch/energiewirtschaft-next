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

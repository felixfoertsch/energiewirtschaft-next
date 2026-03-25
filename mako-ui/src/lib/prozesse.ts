export interface ProzessSchrittDef {
	name: string;
	absender: string;
	empfaenger: string;
	typ: string;
	nachrichtentyp: string;
}

export interface ProzessDef {
	key: string;
	name: string;
	kategorie: "GPKE" | "WiM" | "UBP" | "MaBiS" | "Abrechnung" | "RD 2.0" | "14a" | "GeLi Gas" | "GABi Gas" | "KoV";
	schritte: ProzessSchrittDef[];
}

export const PROZESSE: ProzessDef[] = [
	// --- GPKE (Strom) ---
	{
		key: "gpke_lfw",
		name: "Lieferantenwechsel",
		kategorie: "GPKE",
		schritte: [
			{ name: "Anmeldung", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "UtilmdAnmeldung", nachrichtentyp: "UTILMD" },
			{ name: "Bestätigung", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdBestaetigung", nachrichtentyp: "UTILMD" },
			{ name: "Abmeldung an LFA", absender: "netzbetreiber", empfaenger: "lieferant_alt", typ: "UtilmdAbmeldung", nachrichtentyp: "UTILMD" },
			{ name: "Widerspruchsfrist", absender: "lieferant_alt", empfaenger: "netzbetreiber", typ: "intern", nachrichtentyp: "" },
			{ name: "Zuordnung an LFN", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdZuordnung", nachrichtentyp: "UTILMD" },
			{ name: "Zuordnung an LFA", absender: "netzbetreiber", empfaenger: "lieferant_alt", typ: "UtilmdZuordnung", nachrichtentyp: "UTILMD" },
		],
	},
	{
		key: "gpke_lieferende",
		name: "Lieferende",
		kategorie: "GPKE",
		schritte: [
			{ name: "Abmeldung", absender: "lieferant_alt", empfaenger: "netzbetreiber", typ: "UtilmdLieferendeAbmeldung", nachrichtentyp: "UTILMD" },
			{ name: "Bestätigung", absender: "netzbetreiber", empfaenger: "lieferant_alt", typ: "UtilmdLieferendeBestaetigung", nachrichtentyp: "UTILMD" },
			{ name: "Schlussturnusmesswert", absender: "messstellenbetreiber", empfaenger: "netzbetreiber", typ: "MsconsSchlussturnusmesswert", nachrichtentyp: "MSCONS" },
		],
	},
	{
		key: "gpke_stammdaten",
		name: "Stammdatenänderung",
		kategorie: "GPKE",
		schritte: [
			{ name: "Änderung senden", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdStammdatenaenderung", nachrichtentyp: "UTILMD" },
			{ name: "Bestätigung/Ablehnung", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "UtilmdBestaetigung", nachrichtentyp: "UTILMD" },
		],
	},
	{
		key: "gpke_gda",
		name: "Geschäftsdatenanfrage",
		kategorie: "GPKE",
		schritte: [
			{ name: "Anfrage", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "UtilmdGeschaeftsdatenanfrage", nachrichtentyp: "UTILMD" },
			{ name: "Antwort", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdGeschaeftsdatenantwort", nachrichtentyp: "UTILMD" },
		],
	},
	{
		key: "gpke_zuordnung",
		name: "Zuordnungsliste",
		kategorie: "GPKE",
		schritte: [
			{ name: "Liste versenden", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdZuordnungsliste", nachrichtentyp: "UTILMD" },
		],
	},

	// --- WiM ---
	{
		key: "wim_msb_wechsel",
		name: "MSB-Wechsel",
		kategorie: "WiM",
		schritte: [
			{ name: "Anmeldung MSB neu", absender: "messstellenbetreiber", empfaenger: "netzbetreiber", typ: "UtilmdMsbWechselAnmeldung", nachrichtentyp: "UTILMD" },
			{ name: "Abmeldung MSB alt", absender: "netzbetreiber", empfaenger: "messstellenbetreiber", typ: "UtilmdAbmeldung", nachrichtentyp: "UTILMD" },
			{ name: "Bestätigung", absender: "netzbetreiber", empfaenger: "messstellenbetreiber", typ: "UtilmdBestaetigung", nachrichtentyp: "UTILMD" },
		],
	},
	{
		key: "wim_zaehlwert",
		name: "Zählwertübermittlung",
		kategorie: "WiM",
		schritte: [
			{ name: "Lastgang MSB → NB", absender: "messstellenbetreiber", empfaenger: "netzbetreiber", typ: "MsconsLastgang", nachrichtentyp: "MSCONS" },
			{ name: "Lastgang NB → LF", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "MsconsLastgang", nachrichtentyp: "MSCONS" },
		],
	},

	// --- UBP ---
	{
		key: "ubp_bestellung",
		name: "Bestellung Messprodukt",
		kategorie: "UBP",
		schritte: [
			{ name: "Angebotsanfrage", absender: "lieferant_neu", empfaenger: "messstellenbetreiber", typ: "ReqoteAngebotsanfrage", nachrichtentyp: "REQOTE" },
			{ name: "Angebot", absender: "messstellenbetreiber", empfaenger: "lieferant_neu", typ: "QuotesAngebot", nachrichtentyp: "QUOTES" },
			{ name: "Bestellung", absender: "lieferant_neu", empfaenger: "messstellenbetreiber", typ: "OrdersBestellung", nachrichtentyp: "ORDERS" },
			{ name: "Bestellantwort", absender: "messstellenbetreiber", empfaenger: "lieferant_neu", typ: "OrdrspBestellantwort", nachrichtentyp: "ORDRSP" },
		],
	},

	// --- MaBiS ---
	{
		key: "mabis_bilanzkreiszuordnung",
		name: "Bilanzkreiszuordnung",
		kategorie: "MaBiS",
		schritte: [
			{ name: "Zuordnung", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "UtilmdBilanzkreiszuordnung", nachrichtentyp: "UTILMD" },
			{ name: "Bestätigung", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdBestaetigung", nachrichtentyp: "UTILMD" },
		],
	},
	{
		key: "mabis_mehrmindermengen",
		name: "Mehr-/Mindermengen",
		kategorie: "MaBiS",
		schritte: [
			{ name: "Liste", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "MsconsMehrMindermengen", nachrichtentyp: "MSCONS" },
			{ name: "Rechnung", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "InvoicRechnung", nachrichtentyp: "INVOIC" },
			{ name: "Zahlungsavis", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "RemadvZahlungsavis", nachrichtentyp: "REMADV" },
		],
	},

	// --- Abrechnung ---
	{
		key: "abrechnung_netznutzung",
		name: "Netznutzungsrechnung",
		kategorie: "Abrechnung",
		schritte: [
			{ name: "Rechnung", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "InvoicRechnung", nachrichtentyp: "INVOIC" },
			{ name: "Zahlungsavis", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "RemadvZahlungsavis", nachrichtentyp: "REMADV" },
		],
	},

	// --- RD 2.0 ---
	{
		key: "rd2_abruf",
		name: "Redispatch-Abruf",
		kategorie: "RD 2.0",
		schritte: [
			{ name: "Aktivierung", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "RdAktivierung", nachrichtentyp: "XML" },
			{ name: "Quittierung", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "RdQuittung", nachrichtentyp: "XML" },
		],
	},
	{
		key: "rd2_stammdaten",
		name: "RD-Stammdaten",
		kategorie: "RD 2.0",
		schritte: [
			{ name: "Stammdaten senden", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "RdStammdaten", nachrichtentyp: "XML" },
			{ name: "Bestätigung", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "RdBestaetigung", nachrichtentyp: "XML" },
		],
	},

	// --- 14a ---
	{
		key: "14a_steuerung",
		name: "§14a Steuerung",
		kategorie: "14a",
		schritte: [
			{ name: "Anmeldung SVE", absender: "netzbetreiber", empfaenger: "messstellenbetreiber", typ: "UtilmdSteuerbareVerbrauchseinrichtung", nachrichtentyp: "UTILMD" },
			{ name: "Steuersignal", absender: "netzbetreiber", empfaenger: "messstellenbetreiber", typ: "ClsSteuersignal", nachrichtentyp: "CLS" },
		],
	},

	// --- GeLi Gas ---
	{
		key: "geli_lfw",
		name: "Lieferantenwechsel Gas",
		kategorie: "GeLi Gas",
		schritte: [
			{ name: "Anmeldung", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "UtilmdAnmeldung", nachrichtentyp: "UTILMD" },
			{ name: "Bestätigung", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdBestaetigung", nachrichtentyp: "UTILMD" },
			{ name: "Abmeldung an LFA", absender: "netzbetreiber", empfaenger: "lieferant_alt", typ: "UtilmdAbmeldung", nachrichtentyp: "UTILMD" },
			{ name: "Zuordnung an LFN", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdZuordnung", nachrichtentyp: "UTILMD" },
			{ name: "Zuordnung an LFA", absender: "netzbetreiber", empfaenger: "lieferant_alt", typ: "UtilmdZuordnung", nachrichtentyp: "UTILMD" },
		],
	},

	// --- GABi Gas ---
	{
		key: "gabi_nominierung",
		name: "Nominierung",
		kategorie: "GABi Gas",
		schritte: [
			{ name: "Nominierung", absender: "bilanzkreisverantwortlicher", empfaenger: "marktgebietsverantwortlicher", typ: "Nominierung", nachrichtentyp: "MSCONS" },
			{ name: "Bestätigung", absender: "marktgebietsverantwortlicher", empfaenger: "bilanzkreisverantwortlicher", typ: "NominierungBestaetigung", nachrichtentyp: "MSCONS" },
		],
	},

	// --- KoV ---
	{
		key: "kov_brennwert",
		name: "Brennwertmitteilung",
		kategorie: "KoV",
		schritte: [
			{ name: "Brennwert mitteilen", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "MsconsBrennwert", nachrichtentyp: "MSCONS" },
		],
	},
];

export const KATEGORIEN = [
	"GPKE",
	"WiM",
	"UBP",
	"MaBiS",
	"Abrechnung",
	"RD 2.0",
	"14a",
	"GeLi Gas",
	"GABi Gas",
	"KoV",
] as const;

export function prozesseFuerRolle(rolle: string): ProzessDef[] {
	return PROZESSE.filter((p) =>
		p.schritte.some((s) => s.absender === rolle || s.empfaenger === rolle),
	);
}

export function naechsterSchritt(prozess: ProzessDef, rolle: string): ProzessSchrittDef | undefined {
	return prozess.schritte.find((s) => s.absender === rolle);
}

// Welt "Rheinland 2026"
//
// Die Welt lebt im Frontend; die Prozesse leben in Rust. Diese Datei stellt
// allen UI-Komponenten benannte Marktteilnehmer (Personas), reale
// Marktlokationen, Beziehungen und Default-Werte bereit, damit eine
// fachliche Person die Simulation als zusammenhängende Geschichte
// nachvollziehen kann.

import type { RJSFSchema } from "@rjsf/utils";

// MP-IDs entsprechen exakt mako-cli/src/init.rs::mp_id_for_index. Index = Reihenfolge in
// init.rs::ROLLEN. Append-only.
const ROLLEN_INDEX: Record<string, number> = {
	lieferant_neu: 0,
	netzbetreiber: 1,
	lieferant_alt: 2,
	messstellenbetreiber: 3,
	bilanzkreisverantwortlicher: 4,
	marktgebietsverantwortlicher: 5,
	lieferant_ersatz_grundversorgung: 6,
	netzbetreiber_alt: 7,
	netzbetreiber_neu: 8,
	messstellenbetreiber_alt: 9,
	messstellenbetreiber_neu: 10,
	grundzustaendiger_messstellenbetreiber: 11,
	messdienstleister: 12,
	uebertragungsnetzbetreiber: 13,
	bilanzkoordinator: 14,
	anschlussnetzbetreiber: 15,
	anfordernder_netzbetreiber: 16,
	wettbewerblicher_messstellenbetreiber: 17,
	einsatzverantwortlicher: 18,
	betreiber_technische_ressource: 19,
	data_provider: 20,
	betreiber_erzeugungsanlage: 21,
	direktvermarkter: 22,
	energieserviceanbieter: 23,
	aggregator: 24,
	ladepunktbetreiber: 25,
	registerbetreiber_hknr: 26,
	fernleitungsnetzbetreiber: 27,
	transportkunde: 28,
	kapazitaetsnutzer: 29,
	speicherstellenbetreiber: 30,
	einspeisenetzbetreiber: 31,
	ausspeisenetzbetreiber: 32,
};

const MP_BASE = 9_900_000_000_000n;

export function mpIdForRolle(rolle: string): string | undefined {
	const idx = ROLLEN_INDEX[rolle];
	if (idx === undefined) return undefined;
	return (MP_BASE + BigInt(idx)).toString().padStart(13, "0");
}

// ───────────────────────────────────────────────────────────────────────────
// Personas
// ───────────────────────────────────────────────────────────────────────────

export interface Persona {
	rolle: string;
	rollenKuerzel: string;
	vorname: string;
	nachname: string;
	firma: string;
	mpId: string;
	beschreibung: string;
}

function persona(
	rolle: string,
	rollenKuerzel: string,
	vorname: string,
	nachname: string,
	firma: string,
	beschreibung: string,
): Persona {
	const mpId = mpIdForRolle(rolle);
	if (!mpId) throw new Error(`Unbekannte Rolle: ${rolle}`);
	return { rolle, rollenKuerzel, vorname, nachname, firma, mpId, beschreibung };
}

export const PERSONAS: Persona[] = [
	persona(
		"lieferant_neu",
		"LFN",
		"Lara",
		"Lieferer",
		"Grünstrom GmbH",
		"Versorgt 12.000 Haushalte mit Ökostrom aus Wind und PV.",
	),
	persona(
		"netzbetreiber",
		"NB",
		"Norbert",
		"Netzwart",
		"Rheinenergie Netz GmbH",
		"Verteilnetzbetreiber im Raum Köln/Bonn, 480.000 Anschlüsse.",
	),
	persona(
		"lieferant_alt",
		"LFA",
		"Ludwig",
		"Liefer",
		"Rheinstrom AG",
		"Etablierter Vollsortimenter, viele Bestandskunden im Rheinland.",
	),
	persona(
		"messstellenbetreiber",
		"MSB",
		"Mira",
		"Messer",
		"SmartMeter West GmbH",
		"Rollt iMSys im Verteilnetz Rheinenergie aus.",
	),
	persona(
		"bilanzkreisverantwortlicher",
		"BKV",
		"Bernd",
		"Bilanz",
		"Bilanzkreis Rhein GmbH",
		"Verantwortet den Strom-Bilanzkreis BKVRH-001.",
	),
	persona(
		"marktgebietsverantwortlicher",
		"MGV",
		"Magnus",
		"Markt",
		"Trading Hub Europe",
		"Marktgebietsverantwortlicher für THE im Gas-Markt.",
	),
	persona(
		"lieferant_ersatz_grundversorgung",
		"E/G",
		"Erika",
		"Ersatz",
		"Stadtwerke Köln",
		"Grund- und Ersatzversorger im Netzgebiet Köln.",
	),
	persona(
		"netzbetreiber_alt",
		"NBA",
		"Nora",
		"Netz-Alt",
		"Westnetz Bonn",
		"Bisheriger Netzbetreiber bei Netzwechsel-Szenarien.",
	),
	persona(
		"netzbetreiber_neu",
		"NBN",
		"Niklas",
		"Netz-Neu",
		"Innogy Netz",
		"Neuer Netzbetreiber bei Netzgebietsübergaben.",
	),
	persona(
		"messstellenbetreiber_alt",
		"MSBA",
		"Marlene",
		"Messer-Alt",
		"AlphaMess GmbH",
		"Vorheriger MSB beim Messstellenbetreiber-Wechsel.",
	),
	persona(
		"messstellenbetreiber_neu",
		"MSBN",
		"Mats",
		"Messer-Neu",
		"BetaMess GmbH",
		"Neuer MSB beim Messstellenbetreiber-Wechsel.",
	),
	persona(
		"grundzustaendiger_messstellenbetreiber",
		"gMSB",
		"Greta",
		"Grundmesser",
		"Rheinenergie Mess",
		"Grundzuständiger MSB im Netzgebiet Rheinenergie.",
	),
	persona(
		"messdienstleister",
		"MDL",
		"Mila",
		"Messdienst",
		"DataFlow Rhein",
		"Auslese- und Messdienstleister für komplexe Messstellen.",
	),
	persona(
		"uebertragungsnetzbetreiber",
		"ÜNB",
		"Ulf",
		"Übertrager",
		"Amprion West",
		"ÜNB für die Regelzone Westdeutschland.",
	),
	persona(
		"bilanzkoordinator",
		"BIKO",
		"Bertha",
		"Bilanz-Koord",
		"ENTSO-E Cluster DE",
		"Bilanzkoordinator für die deutsche Regelzone.",
	),
	persona(
		"anschlussnetzbetreiber",
		"ANB",
		"Anton",
		"Anschluss",
		"Anschlussnetz Rheinland",
		"ANB im RD-2.0-Stammdatenaustausch.",
	),
	persona(
		"anfordernder_netzbetreiber",
		"anfNB",
		"Anneli",
		"Anforderer",
		"Bayernwerk Netz",
		"Anfordernder NB bei kaskadierten Redispatch-Abrufen.",
	),
	persona(
		"wettbewerblicher_messstellenbetreiber",
		"wMSB",
		"Walter",
		"Wettbewerb",
		"SmartGrid Pioneers",
		"Wettbewerblicher MSB mit eigenem iMSys-Portfolio.",
	),
	persona(
		"einsatzverantwortlicher",
		"EIV",
		"Ephraim",
		"Einsatz",
		"Eichler Energiedispatch",
		"Einsatzverantwortlicher für TR-WIND-001 und PV-Park Hürth.",
	),
	persona(
		"betreiber_technische_ressource",
		"BTR",
		"Bruno",
		"Betreiber",
		"Bruno Wind GmbH",
		"Betreibt die technische Ressource TR-WIND-001 bei Bonn.",
	),
	persona(
		"data_provider",
		"DP",
		"Dora",
		"Datenprovider",
		"ConnectedGrid Services",
		"Data Provider für RD-2.0-Stammdatenaustausch und -fahrpläne.",
	),
	persona(
		"betreiber_erzeugungsanlage",
		"BEA",
		"Beate",
		"Erzeuger",
		"Solarpark Hürth GmbH",
		"Betreibt den 8 MW PV-Park Hürth.",
	),
	persona(
		"direktvermarkter",
		"DV",
		"Dorian",
		"Direkt",
		"Greenmarket Trading",
		"Direktvermarkter für PV-Park Hürth.",
	),
	persona(
		"energieserviceanbieter",
		"ESA",
		"Erik",
		"Energieservice",
		"EnergyAdvisor GmbH",
		"Energieserviceanbieter für gewerbliche Kunden.",
	),
	persona(
		"aggregator",
		"AGG",
		"Achim",
		"Aggregator",
		"FlexPool Rhein",
		"Aggregator für Flexibilitäts-Pools (Heimspeicher, Wallboxen).",
	),
	persona(
		"ladepunktbetreiber",
		"LPB",
		"Lena",
		"Ladepunkt",
		"eMobility Rheinland",
		"Ladepunktbetreiber im Großraum Köln.",
	),
	persona(
		"registerbetreiber_hknr",
		"RB-HKNR",
		"Rita",
		"Register",
		"Umweltbundesamt HKNR",
		"Registerbetreiber für Herkunftsnachweise.",
	),
	persona(
		"fernleitungsnetzbetreiber",
		"FNB",
		"Ferdi",
		"Fernleitung",
		"Gascade Transport",
		"Fernleitungsnetzbetreiber im Marktgebiet THE.",
	),
	persona(
		"transportkunde",
		"TK",
		"Tilda",
		"Transport",
		"Trianel Gas Trading",
		"Transportkunde für gewerbliche Gas-Endkunden.",
	),
	persona(
		"kapazitaetsnutzer",
		"KN",
		"Karlo",
		"Kapazität",
		"KapaTrader GmbH",
		"Kapazitätsnutzer im FNB-Netz.",
	),
	persona(
		"speicherstellenbetreiber",
		"SSB",
		"Selma",
		"Speicher",
		"Astora Speicher Rehden",
		"Speicherstellenbetreiber, Gasspeicher Rehden.",
	),
	persona(
		"einspeisenetzbetreiber",
		"ENB",
		"Ennio",
		"Einspeiser",
		"Open Grid Europe",
		"Einspeisenetzbetreiber im Marktgebiet THE.",
	),
	persona(
		"ausspeisenetzbetreiber",
		"ANBG",
		"Anke",
		"Ausspeiser",
		"Thyssengas",
		"Ausspeisenetzbetreiber für regionale Gas-Endkunden.",
	),
];

const PERSONAS_BY_ROLLE = new Map(PERSONAS.map((p) => [p.rolle, p]));
const PERSONAS_BY_MP_ID = new Map(PERSONAS.map((p) => [p.mpId, p]));

export function personaForRolle(rolle: string): Persona | undefined {
	return PERSONAS_BY_ROLLE.get(rolle);
}

export function personaByMpId(mpId: string): Persona | undefined {
	return PERSONAS_BY_MP_ID.get(mpId);
}

// ───────────────────────────────────────────────────────────────────────────
// Marktlokationen mit Beziehungsnetz
// ───────────────────────────────────────────────────────────────────────────

export interface Anschrift {
	strasse: string;
	hausnummer: string;
	plz: string;
	ort: string;
}

export interface MaLo {
	id: string;
	bezeichnung: string;
	story: string;
	sparte: "strom" | "gas";
	anschrift: Anschrift;
	bilanzkreis?: string;
	ressource_id?: string;
	melo_id?: string;
	zaehlernummer?: string;
	beziehungen: Partial<Record<string, string>>; // role-slug → mpId
	kunde?: { vorname: string; nachname: string };
}

function rels(...rollen: string[]): Partial<Record<string, string>> {
	const map: Partial<Record<string, string>> = {};
	for (const rolle of rollen) {
		const id = mpIdForRolle(rolle);
		if (id) map[rolle] = id;
	}
	return map;
}

export const MALOS: MaLo[] = [
	{
		id: "51111111111",
		bezeichnung: "Familie Schmidt, Köln",
		story:
			"Familie Schmidt wechselt zum 1. Mai 2026 von Rheinstrom (Ludwig) zu Grünstrom (Lara). Norbert ist Netzbetreiber, Mira hat das iMSys installiert.",
		sparte: "strom",
		anschrift: { strasse: "Hauptstr.", hausnummer: "1", plz: "50667", ort: "Köln" },
		bilanzkreis: "BKVRH-001",
		melo_id: "DE0001234567890123456789012345678",
		zaehlernummer: "1EMH0012345678",
		kunde: { vorname: "Hans", nachname: "Schmidt" },
		beziehungen: rels(
			"netzbetreiber",
			"lieferant_alt",
			"lieferant_neu",
			"messstellenbetreiber",
			"grundzustaendiger_messstellenbetreiber",
			"bilanzkreisverantwortlicher",
			"lieferant_ersatz_grundversorgung",
		),
	},
	{
		id: "52222222227",
		bezeichnung: "Bäckerei Klein, Bonn (Wind-PPA)",
		story:
			"Bäckerei Klein bezieht Strom direkt aus Brunos Windanlage TR-WIND-001 über einen PPA. Bei Engpässen aktiviert Anneli den Redispatch über Anton ANB → Ephraim EIV → Bruno BTR.",
		sparte: "strom",
		anschrift: { strasse: "Bonner Talweg", hausnummer: "42", plz: "53113", ort: "Bonn" },
		bilanzkreis: "BKVRH-002",
		ressource_id: "TR-WIND-001",
		kunde: { vorname: "Klara", nachname: "Klein" },
		beziehungen: rels(
			"netzbetreiber",
			"anschlussnetzbetreiber",
			"anfordernder_netzbetreiber",
			"einsatzverantwortlicher",
			"betreiber_technische_ressource",
			"data_provider",
			"lieferant_neu",
			"bilanzkreisverantwortlicher",
			"messstellenbetreiber",
		),
	},
	{
		id: "53333333333",
		bezeichnung: "Solarpark Hürth (Erzeugung)",
		story:
			"Beate betreibt den 8 MW Solarpark, Dorian vermarktet direkt. Greta gMSB liest viertelstündlich, Mila MDL fasst die Werte für die Bilanzierung zusammen.",
		sparte: "strom",
		anschrift: { strasse: "Zur Solarwiese", hausnummer: "1", plz: "50354", ort: "Hürth" },
		bilanzkreis: "BKVRH-003",
		ressource_id: "TR-PV-HUERTH",
		kunde: { vorname: "Beate", nachname: "Erzeuger" },
		beziehungen: rels(
			"netzbetreiber",
			"betreiber_erzeugungsanlage",
			"direktvermarkter",
			"messstellenbetreiber",
			"grundzustaendiger_messstellenbetreiber",
			"messdienstleister",
			"bilanzkreisverantwortlicher",
			"uebertragungsnetzbetreiber",
			"bilanzkoordinator",
			"einsatzverantwortlicher",
			"data_provider",
			"anschlussnetzbetreiber",
			"registerbetreiber_hknr",
		),
	},
	{
		id: "54444444449",
		bezeichnung: "Industriebäckerei Kerpen (Gas)",
		story:
			"Industriebäckerei Kerpen bezieht Erdgas. Tilda als Transportkunde nominiert bei Magnus, Anke speist aus, Selma puffert in Rehden, Ferdi transportiert.",
		sparte: "gas",
		anschrift: { strasse: "Industriestr.", hausnummer: "12", plz: "50171", ort: "Kerpen" },
		kunde: { vorname: "Konrad", nachname: "Kerpen" },
		beziehungen: rels(
			"marktgebietsverantwortlicher",
			"fernleitungsnetzbetreiber",
			"transportkunde",
			"kapazitaetsnutzer",
			"speicherstellenbetreiber",
			"einspeisenetzbetreiber",
			"ausspeisenetzbetreiber",
			"messstellenbetreiber",
			"bilanzkreisverantwortlicher",
		),
	},
	{
		id: "55555555550",
		bezeichnung: "Wallbox Familie Schmidt (§14a SVE)",
		story:
			"Familie Schmidt hat eine 11 kW Wallbox als steuerbare Verbrauchseinrichtung. Norbert NB darf bei Engpässen über Lena LPB die Leistung dimmen.",
		sparte: "strom",
		anschrift: { strasse: "Hauptstr.", hausnummer: "1", plz: "50667", ort: "Köln" },
		ressource_id: "SVE-WALLBOX-SCHMIDT",
		kunde: { vorname: "Hans", nachname: "Schmidt" },
		beziehungen: rels(
			"netzbetreiber",
			"ladepunktbetreiber",
			"lieferant_neu",
			"messstellenbetreiber",
			"aggregator",
			"energieserviceanbieter",
		),
	},
];

// ───────────────────────────────────────────────────────────────────────────
// Beziehungen / Adressbuch
// ───────────────────────────────────────────────────────────────────────────

export function malosFuerRolle(rolle: string): MaLo[] {
	return MALOS.filter((malo) => malo.beziehungen[rolle]);
}

export function bestMaLoFuerSchritt(
	absenderRolle: string,
	empfaengerRolle: string,
): MaLo | undefined {
	return (
		MALOS.find((malo) => malo.beziehungen[absenderRolle] && malo.beziehungen[empfaengerRolle]) ??
		malosFuerRolle(absenderRolle)[0]
	);
}

// "Adressbuch" — wer redet typischerweise mit wem? Liefert für eine Sender-/
// Empfänger-Kombination alle Personas, die die Sender-Persona kennt.
export function empfaengerKandidaten(absenderRolle: string, empfaengerRolle: string): Persona[] {
	const absenderMpId = mpIdForRolle(absenderRolle);
	if (!absenderMpId) return [];
	const erreichbar = new Set<string>();
	for (const malo of MALOS) {
		if (malo.beziehungen[absenderRolle] === absenderMpId) {
			const partnerMp = malo.beziehungen[empfaengerRolle];
			if (partnerMp) erreichbar.add(partnerMp);
		}
	}
	const personas = [...erreichbar]
		.map((mpId) => personaByMpId(mpId))
		.filter((p): p is Persona => Boolean(p));
	if (personas.length > 0) return personas;
	const fallback = personaForRolle(empfaengerRolle);
	return fallback ? [fallback] : [];
}

// ───────────────────────────────────────────────────────────────────────────
// FormContext + Schema-basiertes Pre-Filling
// ───────────────────────────────────────────────────────────────────────────

export interface FormContext {
	malo?: MaLo;
	absender: Persona;
	empfaenger: Persona;
	heute: string;
	morgen: string;
	in_dreissig_tagen: string;
}

export function buildContext(
	absenderRolle: string,
	empfaenger: Persona,
	malo?: MaLo,
): FormContext | null {
	const absender = personaForRolle(absenderRolle);
	if (!absender) return null;
	const heute = new Date();
	heute.setHours(0, 0, 0, 0);
	const morgen = new Date(heute.getTime() + 24 * 60 * 60 * 1000);
	const in30 = new Date(heute.getTime() + 30 * 24 * 60 * 60 * 1000);
	return {
		malo,
		absender,
		empfaenger,
		heute: isoDate(heute),
		morgen: isoDate(morgen),
		in_dreissig_tagen: isoDate(in30),
	};
}

function isoDate(d: Date): string {
	const y = d.getFullYear();
	const m = String(d.getMonth() + 1).padStart(2, "0");
	const day = String(d.getDate()).padStart(2, "0");
	return `${y}-${m}-${day}`;
}

// Walks the JSON-Schema and fills field values from FormContext where the
// field name matches a known German MaKo identifier. Pure function, returns a
// new object — never mutates inputs.
export function seedFormDefaults(
	schema: RJSFSchema,
	context: FormContext,
): Record<string, unknown> {
	return seedObject(schema, schema, context, {
		heute: context.heute,
		morgen: context.morgen,
		in30: context.in_dreissig_tagen,
	});
}

interface DateBag {
	heute: string;
	morgen: string;
	in30: string;
}

interface SchemaObject {
	$ref?: string;
	default?: unknown;
	definitions?: Record<string, RJSFSchema>;
	enum?: unknown[];
	format?: string;
	items?: RJSFSchema | RJSFSchema[];
	minItems?: number;
	oneOf?: RJSFSchema[];
	anyOf?: RJSFSchema[];
	pattern?: string;
	properties?: Record<string, RJSFSchema>;
	required?: string[];
	type?: string | string[];
}

function seedObject(
	schema: RJSFSchema,
	root: RJSFSchema,
	ctx: FormContext,
	dates: DateBag,
): Record<string, unknown> {
	const resolved = resolveSchema(schema, root);
	const props = asSchemaObject(resolved).properties ?? {};
	const required = new Set(asSchemaObject(resolved).required ?? []);
	const out: Record<string, unknown> = {};

	for (const [key, prop] of Object.entries(props)) {
		const value = defaultForField(key, prop, ctx, dates, root, required.has(key));
		if (value !== undefined) out[key] = value;
	}

	return out;
}

function defaultForField(
	rawKey: string,
	prop: RJSFSchema,
	ctx: FormContext,
	dates: DateBag,
	root: RJSFSchema,
	required: boolean,
): unknown {
	const resolved = resolveSchema(prop, root);
	const schema = asSchemaObject(resolved);
	const worldDefault = defaultForKnownField(rawKey, resolved, ctx, dates);
	if (worldDefault !== undefined) return worldDefault;
	if (Object.hasOwn(schema, "default")) return cloneJsonValue(schema.default);

	const enumValue = enumDefault(resolved);
	if (enumValue !== undefined) return enumValue;

	const type = schemaType(resolved);
	if (type === "object" || schema.properties) return seedObject(resolved, root, ctx, dates);
	if (type === "array" || schema.items)
		return arrayDefault(rawKey, resolved, ctx, dates, root, required);

	const optionDefault = schemaOptionDefault(rawKey, resolved, ctx, dates, root, required);
	if (optionDefault !== undefined) return optionDefault;

	if (type === "boolean") return false;
	if (type === "integer" || type === "number") return required ? 1 : undefined;
	if (type === "string" || schema.format || schema.pattern) {
		return stringFallback(rawKey, resolved, ctx, dates, required);
	}

	return required ? "BEISPIEL" : undefined;
}

function defaultForKnownField(
	rawKey: string,
	prop: RJSFSchema,
	ctx: FormContext,
	dates: DateBag,
): unknown {
	const key = rawKey.toLowerCase();
	const compactKey = key.replaceAll("_", "").replaceAll("-", "");
	const malo = ctx.malo;

	if (key in ROLLEN_INDEX) return malo?.beziehungen[key] ?? mpIdForRolle(key);
	if (key === "mp_id" || key === "marktpartner_id" || key === "marktpartnerid")
		return ctx.absender.mpId;
	if (key === "malo_id" || key === "marktlokations_id") return malo?.id ?? MALOS[0]?.id;
	if (key === "melo_id" || key === "messlokations_id")
		return malo?.melo_id ?? "DE0001234567890123456789012345678";
	if (key === "zaehlernummer" || key === "geraetenummer" || key === "zaehler_id")
		return malo?.zaehlernummer ?? "1EMH0012345678";
	if (key === "bilanzkreis" || key === "bk_id")
		return malo?.bilanzkreis ?? (malo?.sparte === "gas" ? "GAS-THE-RH-001" : "BKVRH-001");
	if (key === "ressource_id" || key === "tr_id") return malo?.ressource_id ?? "TR-PV-HUERTH";
	if (key === "tranche_id") return "TRANCHE-RH-2026-001";
	if (key === "lastgang_id") return "LG-RH-2026-0001";
	if (key === "vertragskontonummer") return `VK-${malo?.id ?? "51111111111"}`;
	if (key === "pruefidentifikator" || key === "prüfidentifikator") return "11042";
	if (key === "dvgw_nr" || key === "dvgw_nummer") return "9870078900001";
	if (key === "nominierungs_id") return "NOM-THE-2026-0001";
	if (key === "absender" || key === "absender_id" || key === "absender_mp_id")
		return ctx.absender.mpId;
	if (key === "empfaenger" || key === "empfaenger_id" || key === "empfaenger_mp_id")
		return ctx.empfaenger.mpId;

	if (key === "vorname") return malo?.kunde?.vorname ?? "Hans";
	if (key === "nachname") return malo?.kunde?.nachname ?? "Schmidt";
	if (key === "name" || key === "firmenname" || key === "firma") return ctx.absender.firma;
	if (key === "kundenname") {
		const vorname = malo?.kunde?.vorname ?? "Hans";
		const nachname = malo?.kunde?.nachname ?? "Schmidt";
		return `${vorname} ${nachname}`;
	}
	if (key === "geburtsdatum") return "1985-04-12";
	if (key === "email" || key === "e_mail") return "hans.schmidt@example.test";
	if (key === "telefon" || key === "telefonnummer") return "+492211234567";
	if (key === "strasse") return malo?.anschrift.strasse ?? MALOS[0]?.anschrift.strasse;
	if (key === "hausnummer") return malo?.anschrift.hausnummer ?? MALOS[0]?.anschrift.hausnummer;
	if (key === "postleitzahl" || key === "plz")
		return malo?.anschrift.plz ?? MALOS[0]?.anschrift.plz;
	if (key === "ort") return malo?.anschrift.ort ?? MALOS[0]?.anschrift.ort;

	if (key.includes("netzgebiet"))
		return key.endsWith("_id") ? "RHEINLAND-KOELN" : "Netzgebiet Rheinland";
	if (key === "ressource_typ" || key === "ressourcetyp")
		return enumDefault(prop, "TechnischeRessource") ?? "TechnischeRessource";
	if (key === "sparte") return enumDefault(prop, malo?.sparte === "gas" ? "Gas" : "Strom");
	if (key === "transaktionsgrund") return enumDefault(prop) ?? "Lieferantenwechsel";
	if (key === "grund") return "Wartung";
	if (key === "anlass") return enumDefault(prop) ?? "Stammdatenänderung";
	if (key === "rolle") return ctx.absender.rolle;
	if (key === "rechnungstyp") return enumDefault(prop, "Netznutzung") ?? "Netznutzung";
	if (key === "rechnungsnummer") return "RE-2026-0001";
	if (key === "bezeichnung") return "Netznutzung Familie Schmidt";
	if (key === "status") return enumDefault(prop, "Gemessen") ?? "Gemessen";
	if (key === "einheit") return "kWh";

	if (
		key === "vertragsbeginn" ||
		key === "lieferbeginn" ||
		key === "anmeldedatum" ||
		key === "datum_ab" ||
		key === "beginn"
	) {
		return formatForType(prop, dates.morgen);
	}
	if (
		key === "vertragsende" ||
		key === "lieferende" ||
		key === "abmeldedatum" ||
		key === "datum_bis" ||
		key === "ende"
	) {
		return formatForType(prop, dates.in30);
	}
	if (key === "start" || key === "engpass_start" || key === "von")
		return formatForType(prop, dates.morgen);
	if (key === "engpass_ende" || key === "bis") return formatForType(prop, dates.in30);
	if (key === "stichtag" || key === "datum" || key === "zeitpunkt")
		return formatForType(prop, dates.heute);
	if (key === "erstellungsdatum") return formatForType(prop, dates.heute);
	if (key === "ausfuehrungsdatum" || key === "ausführungsdatum")
		return formatForType(prop, dates.morgen);
	if (key === "gueltig_ab" || key === "gültig_ab") return formatForType(prop, dates.morgen);
	if (key === "gueltig_bis" || key === "gültig_bis") return formatForType(prop, dates.in30);
	if (key === "valuta" || key === "rechnungsdatum" || key === "meldetag")
		return formatForType(prop, dates.heute);
	if (key === "faelligkeit" || key === "fälligkeit") return formatForType(prop, dates.in30);
	if (key.startsWith("lieferzeitraum_")) {
		return key.endsWith("_bis")
			? formatForType(prop, dates.in30)
			: formatForType(prop, dates.morgen);
	}
	if (endsWithDateStart(compactKey)) return formatForType(prop, dates.morgen);
	if (endsWithDateEnd(compactKey)) return formatForType(prop, dates.in30);
	if (endsWithDatePoint(compactKey)) return formatForType(prop, dates.heute);

	if (key === "intervall_minuten") return 15;
	if (key === "sollwert_kw") return 2500.0;
	if (key === "installierte_leistung_kw") return 8000.0;
	if (key === "betroffene_leistung_kw") return 1000.0;
	// Wallbox-typische 11 kW vs. größere Erzeugungsanlagen — orientiert sich an MaLo-Ressource.
	if (key === "max_leistung_kw" || key === "leistung_kw") {
		if (malo?.ressource_id?.includes("WALLBOX")) return 11.0;
		if (malo?.ressource_id?.includes("PV")) return 8000.0;
		if (malo?.ressource_id?.includes("WIND")) return 3000.0;
		return 11.0;
	}
	if (key === "peak_kw") return 8.0;
	if (key === "verbrauch_kwh" || key === "verbrauch")
		return malo?.sparte === "gas" ? 18500.0 : 3500.0;
	if (key === "arbeit_kwh") return 3500.0;
	if (key === "zaehlerstand") return 18250.6;
	if (key === "wert") return 12.5;
	if (key === "menge") return 3500.0;

	if (key === "gesamtbetrag_ct") return 15469;
	if (key === "betrag_ct") return 12999;
	if (key === "einzelpreis_ct") return 32;
	if (key === "grundpreis") return 12.5;
	if (key === "arbeitspreis") return 0.32;
	if (key === "nettobetrag") return 129.99;
	if (key === "bruttobetrag") return 154.69;
	if (key === "mwst") return 24.7;
	if (key === "steuersatz") return 19.0;
	if (key === "rabatt") return 5.0;
	if (key === "betrag" || key === "preis" || key === "summe") return 129.99;

	if (key === "obis_code" || key === "obis_kennzahl") return "1-1:1.8.0";
	if (key === "tarifart") return enumDefault(prop, "Eintarif") ?? "Eintarif";
	if (key === "messverfahren") return enumDefault(prop, "RLM") ?? "RLM";
	if (key === "messlokationstyp") return enumDefault(prop, "Standard") ?? "Standard";
	if (key === "bilanzierungsverfahren")
		return enumDefault(prop, "Standardlastprofil") ?? "Standardlastprofil";
	if (key === "lastprofiltyp") return enumDefault(prop, "H0") ?? "H0";
	if (key === "ueberragungstyp" || key === "uebertragungstyp")
		return enumDefault(prop) ?? "Elektronisch";

	if (key === "brennwert") return 11.2;
	if (key === "zustandszahl") return 0.965;
	if (key === "nominierungsmenge") return 12000.0;
	if (key === "kapazitaet" || key === "kapazität") return 5000.0;
	if (key === "druck") return 16.0;
	if (key === "temperatur") return 15.0;

	if (key === "dimmstufe" || key === "prozent") return 60;
	if (key === "regelstufe") return 2;
	if (key === "dauer_minuten") return 15;
	if (key === "aktivierungstyp") return enumDefault(prop, "Dimmung") ?? "Dimmung";
	if (key === "geraetetyp" || key === "gerätetyp") return enumDefault(prop, "Wallbox") ?? "Wallbox";
	if (key === "steuerung") return enumDefault(prop, "Freigabe") ?? "Freigabe";

	if (key === "aktivierungsart") return enumDefault(prop, "Redispatch") ?? "Redispatch";
	if (key === "richtung") return enumDefault(prop, "Senkung") ?? "Senkung";
	if (key === "aktivierungsgrund") return enumDefault(prop) ?? "Netzengpass Rheinland";
	if (key === "kostenhoehe" || key === "kostenhöhe") return 250.0;

	return undefined;
}

function enumDefault(prop: RJSFSchema, preferred?: string): unknown {
	const values = enumValues(prop);
	if (values.length === 0) return undefined;
	if (preferred && values.includes(preferred)) return preferred;
	return values[0];
}

function formatForType(prop: RJSFSchema, date: string): string {
	const fmt = asSchemaObject(prop).format;
	if (fmt === "date-time" || fmt === "partial-date-time") return `${date} 08:00`;
	if (fmt === "time") return "08:00";
	return date;
}

function resolveSchema(schema: RJSFSchema, root: RJSFSchema): RJSFSchema {
	const ref = asSchemaObject(schema).$ref;
	if (!ref) return schema;
	const resolved = resolveRef(root, ref);
	if (!resolved) return schema;
	const { $ref: _ref, ...siblings } = asSchemaObject(schema);
	void _ref;
	return { ...resolved, ...siblings } as RJSFSchema;
}

function resolveRef(root: RJSFSchema, ref: string): RJSFSchema | undefined {
	if (!ref.startsWith("#/")) return undefined;
	const parts = ref
		.slice(2)
		.split("/")
		.map((part) => part.replaceAll("~1", "/").replaceAll("~0", "~"));
	let current: unknown = root;
	for (const part of parts) {
		if (!isRecord(current)) return undefined;
		current = current[part];
	}
	return isRecord(current) ? (current as RJSFSchema) : undefined;
}

function arrayDefault(
	rawKey: string,
	prop: RJSFSchema,
	ctx: FormContext,
	dates: DateBag,
	root: RJSFSchema,
	required: boolean,
): unknown[] {
	const schema = asSchemaObject(prop);
	if (!required && (schema.minItems ?? 0) < 1) return [];
	const item = Array.isArray(schema.items) ? schema.items[0] : schema.items;
	if (!item) return [];
	return [defaultForField(rawKey, item, ctx, dates, root, true)];
}

function schemaOptionDefault(
	rawKey: string,
	prop: RJSFSchema,
	ctx: FormContext,
	dates: DateBag,
	root: RJSFSchema,
	required: boolean,
): unknown {
	const schema = asSchemaObject(prop);
	const options = schema.oneOf ?? schema.anyOf ?? [];
	for (const option of options) {
		const value = defaultForField(rawKey, option, ctx, dates, root, required);
		if (value !== undefined) return value;
	}
	return undefined;
}

function stringFallback(
	rawKey: string,
	prop: RJSFSchema,
	ctx: FormContext,
	dates: DateBag,
	required: boolean,
): string {
	const schema = asSchemaObject(prop);
	if (schema.format === "date") return dateForField(rawKey, prop, dates);
	if (schema.format === "date-time" || schema.format === "partial-date-time")
		return dateForField(rawKey, prop, dates);
	if (schema.format === "time") return "08:00";
	if (schema.pattern) return patternDefault(schema.pattern, ctx);
	return required ? "BEISPIEL" : "";
}

function patternDefault(pattern: string, ctx: FormContext): string {
	if (pattern.includes("\\d{13}") || pattern.includes("[0-9]{13}")) return ctx.absender.mpId;
	if (pattern.includes("\\d{11}") || pattern.includes("[0-9]{11}"))
		return ctx.malo?.id ?? "51111111111";
	if (pattern.includes("\\d{32}") || pattern.includes("[0-9]{32}"))
		return ctx.malo?.melo_id ?? "DE0001234567890123456789012345678";
	return "0";
}

function dateForField(rawKey: string, prop: RJSFSchema, dates: DateBag): string {
	const compactKey = rawKey.toLowerCase().replaceAll("_", "").replaceAll("-", "");
	if (endsWithDateStart(compactKey)) return formatForType(prop, dates.morgen);
	if (endsWithDateEnd(compactKey)) return formatForType(prop, dates.in30);
	return formatForType(prop, dates.heute);
}

function endsWithDateStart(compactKey: string): boolean {
	return (
		compactKey.endsWith("von") ||
		compactKey.endsWith("ab") ||
		compactKey.endsWith("beginn") ||
		compactKey.endsWith("start")
	);
}

function endsWithDateEnd(compactKey: string): boolean {
	return compactKey.endsWith("bis") || compactKey.endsWith("ende");
}

function endsWithDatePoint(compactKey: string): boolean {
	return compactKey.endsWith("datum") || compactKey.endsWith("zeitpunkt");
}

function enumValues(prop: RJSFSchema): unknown[] {
	const schema = asSchemaObject(prop);
	if (Array.isArray(schema.enum)) return schema.enum;
	const options = schema.oneOf ?? schema.anyOf ?? [];
	return options.flatMap((option) => enumValues(option));
}

function schemaType(prop: RJSFSchema): string | undefined {
	const type = asSchemaObject(prop).type;
	if (Array.isArray(type)) return type.find((value) => value !== "null");
	return type;
}

function cloneJsonValue(value: unknown): unknown {
	if (value === undefined) return undefined;
	return structuredClone(value);
}

function asSchemaObject(schema: RJSFSchema): SchemaObject {
	return schema as SchemaObject;
}

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === "object" && value !== null;
}

// ───────────────────────────────────────────────────────────────────────────
// Welt-Erzählung
// ───────────────────────────────────────────────────────────────────────────

export const WELT_NAME = "Rheinland 2026";
export const WELT_BESCHREIBUNG =
	"Eine fiktive deutsche Energiewirtschaft im Großraum Köln/Bonn mit 33 Marktrollen, 5 Geschichten und voll funktionsfähiger Marktkommunikation.";

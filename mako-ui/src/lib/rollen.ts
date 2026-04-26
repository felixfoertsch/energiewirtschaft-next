export interface RollenDefinition {
	name: string;
	label: string;
	kuerzel: string;
	farbe: string;
}

// Mirror of mako_types::rolle::MarktRolle plus the slug taxonomy from
// mako-cli/src/init.rs. Keep in sync if either changes.
//
// Farb-Schema:
//   blue/orange/sky    → Lieferanten-Familie (LF/LFN/LFA/E-G)
//   emerald            → Netzbetreiber-Familie (NB/NBA/NBN/ANB/anfNB)
//   teal/cyan          → ÜNB / FNB (höhere Netzebenen)
//   purple/violet      → Messwesen (MSB/MSBA/MSBN/gMSB/wMSB/MDL)
//   rose               → Bilanzierung (BKV/BIKO)
//   amber              → Gas-Markt (MGV/TK/KN/SSB/ENB/ANBG)
//   slate              → Erzeugung (BEA/DV/AGG)
//   fuchsia            → Sonstige Strom (EIV/BTR/DP/ESA/LPB/RB)
//   stone              → abstrakt (Rechnungsersteller/-empfänger)
export const ROLLEN: Record<string, RollenDefinition> = {
	// spartenübergreifend — Lieferanten
	lieferant_neu: { name: "lieferant_neu", label: "Lieferant Neu", kuerzel: "LFN", farbe: "bg-blue-500" },
	lieferant_alt: { name: "lieferant_alt", label: "Lieferant Alt", kuerzel: "LFA", farbe: "bg-orange-500" },
	lieferant_ersatz_grundversorgung: { name: "lieferant_ersatz_grundversorgung", label: "Ersatz-/Grundversorger", kuerzel: "E/G", farbe: "bg-sky-500" },

	// spartenübergreifend — Netzbetreiber
	netzbetreiber: { name: "netzbetreiber", label: "Netzbetreiber", kuerzel: "NB", farbe: "bg-emerald-500" },
	netzbetreiber_alt: { name: "netzbetreiber_alt", label: "Netzbetreiber Alt", kuerzel: "NBA", farbe: "bg-emerald-700" },
	netzbetreiber_neu: { name: "netzbetreiber_neu", label: "Netzbetreiber Neu", kuerzel: "NBN", farbe: "bg-emerald-400" },
	anschlussnetzbetreiber: { name: "anschlussnetzbetreiber", label: "Anschlussnetzbetreiber", kuerzel: "ANB", farbe: "bg-emerald-600" },
	anfordernder_netzbetreiber: { name: "anfordernder_netzbetreiber", label: "Anfordernder Netzbetreiber", kuerzel: "anfNB", farbe: "bg-emerald-800" },

	// spartenübergreifend — Messwesen
	messstellenbetreiber: { name: "messstellenbetreiber", label: "Messstellenbetreiber", kuerzel: "MSB", farbe: "bg-purple-500" },
	messstellenbetreiber_alt: { name: "messstellenbetreiber_alt", label: "Messstellenbetreiber Alt", kuerzel: "MSBA", farbe: "bg-purple-700" },
	messstellenbetreiber_neu: { name: "messstellenbetreiber_neu", label: "Messstellenbetreiber Neu", kuerzel: "MSBN", farbe: "bg-purple-400" },
	grundzustaendiger_messstellenbetreiber: { name: "grundzustaendiger_messstellenbetreiber", label: "Grundzuständiger MSB", kuerzel: "gMSB", farbe: "bg-purple-600" },
	wettbewerblicher_messstellenbetreiber: { name: "wettbewerblicher_messstellenbetreiber", label: "Wettbewerblicher MSB", kuerzel: "wMSB", farbe: "bg-violet-500" },
	messdienstleister: { name: "messdienstleister", label: "Messdienstleister", kuerzel: "MDL", farbe: "bg-violet-700" },

	// spartenübergreifend — Bilanzierung
	bilanzkreisverantwortlicher: { name: "bilanzkreisverantwortlicher", label: "Bilanzkreisverantwortlicher", kuerzel: "BKV", farbe: "bg-rose-500" },

	// spartenübergreifend — Abrechnungs-Abstraktion
	rechnungsersteller: { name: "rechnungsersteller", label: "Rechnungsersteller", kuerzel: "ReErst", farbe: "bg-stone-500" },
	rechnungsempfaenger: { name: "rechnungsempfaenger", label: "Rechnungsempfänger", kuerzel: "ReEmpf", farbe: "bg-stone-600" },

	// Strom
	uebertragungsnetzbetreiber: { name: "uebertragungsnetzbetreiber", label: "Übertragungsnetzbetreiber", kuerzel: "ÜNB", farbe: "bg-teal-500" },
	bilanzkoordinator: { name: "bilanzkoordinator", label: "Bilanzkoordinator", kuerzel: "BIKO", farbe: "bg-rose-700" },
	einsatzverantwortlicher: { name: "einsatzverantwortlicher", label: "Einsatzverantwortlicher", kuerzel: "EIV", farbe: "bg-fuchsia-500" },
	betreiber_technische_ressource: { name: "betreiber_technische_ressource", label: "Betreiber Techn. Ressource", kuerzel: "BTR", farbe: "bg-fuchsia-600" },
	data_provider: { name: "data_provider", label: "Data Provider", kuerzel: "DP", farbe: "bg-fuchsia-700" },
	betreiber_erzeugungsanlage: { name: "betreiber_erzeugungsanlage", label: "Betreiber Erzeugungsanlage", kuerzel: "BEA", farbe: "bg-slate-500" },
	direktvermarkter: { name: "direktvermarkter", label: "Direktvermarkter", kuerzel: "DV", farbe: "bg-slate-600" },
	energieserviceanbieter: { name: "energieserviceanbieter", label: "Energieserviceanbieter", kuerzel: "ESA", farbe: "bg-fuchsia-400" },
	aggregator: { name: "aggregator", label: "Aggregator", kuerzel: "AGG", farbe: "bg-slate-700" },
	ladepunktbetreiber: { name: "ladepunktbetreiber", label: "Ladepunktbetreiber", kuerzel: "LPB", farbe: "bg-fuchsia-800" },
	registerbetreiber_hknr: { name: "registerbetreiber_hknr", label: "Registerbetreiber HKNR", kuerzel: "RB-HKNR", farbe: "bg-pink-700" },

	// Gas
	fernleitungsnetzbetreiber: { name: "fernleitungsnetzbetreiber", label: "Fernleitungsnetzbetreiber", kuerzel: "FNB", farbe: "bg-cyan-600" },
	marktgebietsverantwortlicher: { name: "marktgebietsverantwortlicher", label: "Marktgebietsverantwortlicher", kuerzel: "MGV", farbe: "bg-amber-500" },
	transportkunde: { name: "transportkunde", label: "Transportkunde", kuerzel: "TK", farbe: "bg-amber-600" },
	kapazitaetsnutzer: { name: "kapazitaetsnutzer", label: "Kapazitätsnutzer", kuerzel: "KN", farbe: "bg-amber-700" },
	speicherstellenbetreiber: { name: "speicherstellenbetreiber", label: "Speicherstellenbetreiber", kuerzel: "SSB", farbe: "bg-yellow-600" },
	einspeisenetzbetreiber: { name: "einspeisenetzbetreiber", label: "Einspeisenetzbetreiber", kuerzel: "ENB", farbe: "bg-yellow-700" },
	ausspeisenetzbetreiber: { name: "ausspeisenetzbetreiber", label: "Ausspeisenetzbetreiber", kuerzel: "ANBG", farbe: "bg-yellow-800" },
};

export function rollenLabel(name: string): string {
	return ROLLEN[name]?.label ?? name;
}

export function rollenKuerzel(name: string): string {
	return ROLLEN[name]?.kuerzel ?? name;
}

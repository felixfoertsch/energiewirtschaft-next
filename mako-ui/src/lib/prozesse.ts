// Pure helpers around the engine-supplied process catalog
// (`/api/prozesse`). The actual data lives in App-level state; this module
// only contains type aliases, label maps, and filter functions so the
// catalog stays a single source of truth in the Rust engine.

import type { ProzessDef, ProzessKategorie, SchrittDef } from "./types.ts";

export type { ProzessDef, ProzessKategorie, SchrittDef };

// Display order for the category headers in the process picker. Anything
// the engine emits that isn't listed here falls through to the bottom in
// alphabetic order via `kategorieLabel`.
export const KATEGORIEN: readonly ProzessKategorie[] = [
	"gpke",
	"geli_gas",
	"wim",
	"ubp",
	"ma_bis",
	"abrechnung",
	"rd2",
	"para14a",
	"gabi_gas",
	"ko_v",
	"mpes",
] as const;

const KATEGORIE_LABEL: Record<ProzessKategorie, string> = {
	gpke: "GPKE",
	geli_gas: "GeLi Gas",
	wim: "WiM",
	ubp: "UBP",
	ma_bis: "MaBiS",
	abrechnung: "Abrechnung",
	rd2: "RD 2.0",
	para14a: "§14a",
	gabi_gas: "GABi Gas",
	ko_v: "KoV",
	mpes: "MPES",
};

export function kategorieLabel(k: ProzessKategorie): string {
	return KATEGORIE_LABEL[k] ?? k;
}

export function prozesseFuerRolle(rolle: string, prozesse: readonly ProzessDef[]): ProzessDef[] {
	return prozesse.filter((p) =>
		p.schritte.some((s) => s.absender === rolle || s.empfaenger === rolle),
	);
}

export function naechsterSchritt(prozess: ProzessDef, rolle: string): SchrittDef | undefined {
	return prozess.schritte.find((s) => s.absender === rolle);
}

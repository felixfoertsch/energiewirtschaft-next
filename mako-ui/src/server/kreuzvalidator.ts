import type { Express, Request, Response } from "express";

// Dynamic import — degrades gracefully if the package is missing or broken
// biome-ignore lint/suspicious/noExplicitAny: third-party untyped package
let transformer: any = null;
let verfuegbar = false;

try {
	const mod = await import("edifact-json-transformer");
	transformer = mod.createTransformer();
	verfuegbar = true;
} catch {
	console.warn("kreuzvalidator: edifact-json-transformer nicht verfügbar — Kreuzvalidierung deaktiviert");
}

export function isAvailable(): boolean {
	return verfuegbar;
}

export interface KreuzvalidierungErgebnis {
	verfuegbar: boolean;
	felder?: Record<string, unknown>;
	fehler?: string;
}

export function kreuzvalidiere(edifactRoh: string): KreuzvalidierungErgebnis {
	if (!verfuegbar || !transformer) {
		return { verfuegbar: false };
	}

	try {
		const ergebnis = transformer.transform(edifactRoh);
		// Extract key fields for cross-validation
		const felder: Record<string, unknown> = {
			nachrichtentyp: ergebnis.metadata?.message_type ?? null,
			nachrichtenname: ergebnis.metadata?.message_name ?? null,
			kategorie: ergebnis.metadata?.category ?? null,
			version: ergebnis.metadata?.version ?? null,
			pruefidentifikator: ergebnis.metadata?.pruefidentifikator?.id ?? null,
			pruefidentifikatorBeschreibung: ergebnis.metadata?.pruefidentifikator?.description ?? null,
			absender: ergebnis.parties?.sender ?? null,
			empfaenger: ergebnis.parties?.receiver ?? null,
			referenznummer: ergebnis.metadata?.reference_number ?? null,
			datum: ergebnis.dates?.message_date ?? null,
			marktlokationen: ergebnis.body?.stammdaten?.marktlokationen ?? [],
			messlokationen: ergebnis.body?.stammdaten?.messlokationen ?? [],
		};

		return { verfuegbar: true, felder };
	} catch (e) {
		return { verfuegbar: true, fehler: String(e) };
	}
}

export function registerRoutes(app: Express, API: string): void {
	app.post(`${API}/kreuzvalidiere`, (req: Request, res: Response) => {
		const { edifact } = req.body ?? {};
		if (typeof edifact !== "string" || edifact.length === 0) {
			res.status(400).json({ error: "edifact string required" });
			return;
		}
		const ergebnis = kreuzvalidiere(edifact);
		res.json(ergebnis);
	});

	app.get(`${API}/kreuzvalidator-status`, (_req: Request, res: Response) => {
		res.json({ verfuegbar: isAvailable() });
	});
}

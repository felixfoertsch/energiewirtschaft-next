import type {
	BatchErgebnis,
	MarktStatus,
	NachrichtMeta,
	ProzessDef,
	Rolle,
	VerifikationsErgebnis,
} from "./types.ts";
import type { RJSFSchema } from "@rjsf/utils";

export interface ErstelleValidiertPayload {
	rolle: string;
	empfaenger: string;
	empfaenger_id: string;
	typ: string;
	fields: Record<string, unknown>;
}

export interface ErstelleValidiertAntwort {
	ok: boolean;
	datei?: string;
	wire_format: string;
	validierung?: VerifikationsErgebnis;
	fehler?: string;
}

// Derive the API base from Vite's `import.meta.env.BASE_URL` so a path-prefixed
// deployment (e.g. base "/ewn/") routes API + SSE through "/ewn/api" while a
// default deployment stays at "/api".
const BASE = `${import.meta.env.BASE_URL.replace(/\/$/, "")}/api`;

async function get<T>(path: string): Promise<T> {
	const res = await fetch(`${BASE}${path}`);
	if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
	return res.json() as Promise<T>;
}

async function post<T>(path: string, body?: unknown): Promise<T> {
	const res = await fetch(`${BASE}${path}`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: body ? JSON.stringify(body) : undefined,
	});
	if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
	return res.json() as Promise<T>;
}

export const api = {
	rollen: () => get<Rolle[]>("/rollen"),
	prozesse: () => get<ProzessDef[]>("/prozesse"),
	schema: (typ: string) => get<RJSFSchema>(`/schema/${encodeURIComponent(typ)}`),
	inbox: (rolle: string) => get<NachrichtMeta[]>(`/rollen/${rolle}/inbox`),
	outbox: (rolle: string) => get<NachrichtMeta[]>(`/rollen/${rolle}/outbox`),
	state: (rolle: string) => get<Record<string, unknown>>(`/rollen/${rolle}/state`),
	nachricht: (rolle: string, box: string, datei: string) =>
		get<{ meta: NachrichtMeta; inhalt: string; edifact?: string }>(
			`/nachrichten/${rolle}/${box}/${datei}`,
		),
	status: () => get<MarktStatus>("/status"),
	sende: (von: string, datei: string, an: string) =>
		post<{ ok: boolean }>("/sende", { von, datei, an }),
	verarbeite: (rolle: string, datei: string) =>
		post<{ ok: boolean; ausgabe: string }>("/verarbeite", { rolle, datei }),
	verarbeiteAlle: (rolle: string) =>
		post<{ ok: boolean; ausgabe: string }>("/verarbeite-alle", { rolle }),
	erstelle: (rolle: string, payload: Record<string, unknown>) =>
		post<{ ok: boolean; datei: string }>(`/nachrichten/${rolle}`, payload),
	erstelleValidiert: (payload: ErstelleValidiertPayload) =>
		post<ErstelleValidiertAntwort>("/erstelle-validiert", payload),
	verifiziere: (rolle: string, box: string, datei: string) =>
		get<VerifikationsErgebnis>(`/verifiziere/${rolle}/${box}/${datei}`),
	verifiziereBatch: (verzeichnis?: string) =>
		post<BatchErgebnis>("/verifiziere-batch", verzeichnis ? { verzeichnis } : undefined),
	kreuzvalidatorStatus: () =>
		get<{ verfuegbar: boolean }>("/kreuzvalidator-status"),
};

export function subscribeEvents(onMessage: (data: unknown) => void): () => void {
	const source = new EventSource(`${BASE}/events`);
	source.onmessage = (e) => {
		onMessage(JSON.parse(e.data));
	};
	return () => source.close();
}

import type { BatchErgebnis, MarktStatus, NachrichtMeta, Rolle, VerifikationsErgebnis } from "./types.ts";

const BASE = "/api";

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

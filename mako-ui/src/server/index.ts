import { execFileSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve, sep } from "node:path";
import { watch } from "chokidar";
import cors from "cors";
import express, { type Request, type Response } from "express";
import { registerRoutes as registerKreuzvalidatorRoutes } from "./kreuzvalidator.ts";

const app = express();
// Default-deny CORS: this server runs on localhost and reads/writes the local
// markt directory and invokes the CLI. We only allow same-origin and explicit
// dev origins from MAKO_ALLOWED_ORIGINS (comma-separated).
const ALLOWED_ORIGINS = (
	process.env.MAKO_ALLOWED_ORIGINS ?? "http://localhost:5173,http://127.0.0.1:5173"
)
	.split(",")
	.map((s) => s.trim())
	.filter(Boolean);
app.use(
	cors({
		origin: (origin, callback) => {
			// Allow same-origin requests (no Origin header) and the configured allowlist.
			if (!origin || ALLOWED_ORIGINS.includes(origin)) {
				callback(null, true);
			} else {
				callback(new Error(`CORS: origin '${origin}' not allowed`));
			}
		},
	}),
);
app.use(express.json());

const MARKT = resolve(process.env.MAKO_MARKT_PATH ?? "markt");
const CLI = resolve(process.env.MAKO_CLI_PATH ?? "../target/debug/mako-cli");

// Route prefix. Default "/api" matches the local-dev shape (vite proxies
// /api → port 3001). When deployed behind an Apache web backend that strips
// a path prefix (e.g. "/ewn/api" → port 3010 with --remove-prefix), set
// API_PREFIX="" so routes mount at the root.
const API = process.env.API_PREFIX ?? "/api";

function cli(args: string[]): string {
	return execFileSync(CLI, args, { encoding: "utf-8", timeout: 10_000 });
}

function cliWithStdin(args: string[], stdin: string): string {
	return execFileSync(CLI, args, {
		encoding: "utf-8",
		timeout: 10_000,
		input: stdin,
	});
}

// Payload type names match Rust enum variants in mako_types::nachricht::
// NachrichtenPayload — strictly PascalCase letters (and a digit suffix in
// the rare case there's e.g. `Rd2*`). Anything else means injection attempt.
const PAYLOAD_TYP = /^[A-Za-z][A-Za-z0-9]*$/;
const MP_ID = /^[0-9]{13}$/;

function readJsonSafe(path: string): unknown {
	if (!existsSync(path)) return {};
	return JSON.parse(readFileSync(path, "utf-8"));
}

function listFiles(dir: string): string[] {
	if (!existsSync(dir)) return [];
	return readdirSync(dir)
		.filter((f) => !f.startsWith("."))
		.sort();
}

function param(req: Request, name: string): string {
	const v = req.params[name];
	if (Array.isArray(v)) return v[0];
	return v ?? "";
}

// Strict path-segment validator: alphanumerics, underscore, hyphen, dot, and
// (after the first character) spaces. No slashes, no NUL, no `..`, no leading
// dot. Used to reject path-traversal attempts before joining onto MARKT.
const SAFE_SEGMENT = /^[A-Za-z0-9_\-.][A-Za-z0-9_\-. ]*$/;

function isSafeSegment(s: string): boolean {
	if (!s || s === "." || s === "..") return false;
	if (s.includes("/") || s.includes("\\") || s.includes("\0")) return false;
	if (s.startsWith(".")) return false;
	return SAFE_SEGMENT.test(s);
}

/**
 * Resolve a path under MARKT and confirm it does not escape via `..` or
 * symlinks. Throws on any traversal attempt; callers turn that into HTTP 400.
 */
function safeMarktPath(...segments: string[]): string {
	for (const seg of segments) {
		if (!isSafeSegment(seg)) {
			throw new Error(`unsicheres Pfadsegment: ${JSON.stringify(seg)}`);
		}
	}
	const joined = resolve(MARKT, ...segments);
	if (joined !== MARKT && !joined.startsWith(MARKT + sep)) {
		throw new Error(`Pfad verlässt MARKT: ${joined}`);
	}
	return joined;
}

function badRequest(res: Response, e: unknown): void {
	res.status(400).json({ error: String(e instanceof Error ? e.message : e) });
}

function nachrichtMeta(rolle: string, box: string, datei: string) {
	const filePath = safeMarktPath(rolle, box, datei);
	const statusPath = `${filePath}.status.json`;
	const stat = statSync(filePath);
	const status = readJsonSafe(statusPath);
	return {
		datei,
		typ: datei.replace(/^\d+_/, "").replace(/\.(edi|json)$/, ""),
		absender: "",
		empfaenger: rolle,
		zeitpunkt: stat.mtime.toISOString(),
		status,
	};
}

// --- Routes ---

// Process catalog: one shot at startup, cached for the process lifetime.
// The catalog only changes when the engine is recompiled — restarting the
// server is the same trigger anyway, so a forever-cache is correct.
let prozesseCache: unknown | null = null;
function prozesseFromCli(): unknown {
	if (prozesseCache !== null) return prozesseCache;
	const ausgabe = cli(["prozesse-json"]);
	prozesseCache = JSON.parse(ausgabe);
	return prozesseCache;
}

app.get(`${API}/prozesse`, (_req: Request, res: Response) => {
	try {
		res.json(prozesseFromCli());
	} catch (e) {
		res.status(500).json({ error: `Prozesskatalog nicht ladbar: ${String(e)}` });
	}
});

// Per-payload JSON schema. Cached per typ for the process lifetime — the
// schema only changes when the engine recompiles.
const schemaCache = new Map<string, unknown>();

app.get(`${API}/schema/:typ`, (req: Request, res: Response) => {
	const typ = param(req, "typ");
	if (!PAYLOAD_TYP.test(typ)) {
		badRequest(res, new Error(`ungültiger Payload-Typ: ${JSON.stringify(typ)}`));
		return;
	}
	const cached = schemaCache.get(typ);
	if (cached !== undefined) {
		res.json(cached);
		return;
	}
	try {
		const ausgabe = cli(["schema-json", "--typ", typ]);
		const schema = JSON.parse(ausgabe);
		schemaCache.set(typ, schema);
		res.json(schema);
	} catch (e) {
		res.status(404).json({ error: `Schema für ${typ} nicht verfügbar: ${String(e)}` });
	}
});

// Build a wire message from JSON form data, validate it via the engine
// (parse → codec → AHB), and write to the sender's outbox.
app.post(`${API}/erstelle-validiert`, (req: Request, res: Response) => {
	const { rolle, empfaenger, empfaenger_id, typ, fields } = req.body ?? {};

	if (!isSafeSegment(String(rolle))) {
		badRequest(res, new Error("ungültige Sender-Rolle"));
		return;
	}
	if (!isSafeSegment(String(empfaenger))) {
		badRequest(res, new Error("ungültige Empfänger-Rolle"));
		return;
	}
	if (!PAYLOAD_TYP.test(String(typ))) {
		badRequest(res, new Error(`ungültiger Payload-Typ: ${JSON.stringify(typ)}`));
		return;
	}
	if (!MP_ID.test(String(empfaenger_id))) {
		badRequest(res, new Error("empfaenger_id muss 13-stellige MP-ID sein"));
		return;
	}
	if (typeof fields !== "object" || fields === null) {
		badRequest(res, new Error("fields muss ein Objekt sein"));
		return;
	}

	const stdin = JSON.stringify({
		empfaenger_slug: empfaenger,
		empfaenger_id,
		typ,
		fields,
	});

	try {
		const ausgabe = cliWithStdin(
			["erstelle-nachricht", "--rolle", String(rolle), "--markt", MARKT],
			stdin,
		);
		const result = JSON.parse(ausgabe);
		// Strip the absolute path from `datei` so the frontend gets just the
		// filename it can pass to /api/nachrichten and /api/sende.
		if (typeof result.datei === "string") {
			result.datei = result.datei.split("/").pop() ?? result.datei;
		}
		res.json(result);
	} catch (e) {
		res.status(400).json({
			ok: false,
			wire_format: "",
			fehler: `Erstellen fehlgeschlagen: ${String(e)}`,
		});
	}
});

app.get(`${API}/rollen`, (_req: Request, res: Response) => {
	const dirs = readdirSync(MARKT).filter((d) => {
		const p = join(MARKT, d);
		return statSync(p).isDirectory() && d !== "log";
	});
	const rollenJson = readJsonSafe(join(MARKT, "rollen.json")) as Record<string, string>;
	const slugToMpId = new Map<string, string>();
	for (const [mpId, slug] of Object.entries(rollenJson)) slugToMpId.set(slug, mpId);
	const rollen = dirs.map((name) => ({
		name,
		mp_id: slugToMpId.get(name) ?? "",
		verzeichnis: name,
	}));
	res.json(rollen);
});

app.get(`${API}/rollen/:rolle/inbox`, (req: Request, res: Response) => {
	try {
		const rolle = param(req, "rolle");
		const dir = safeMarktPath(rolle, "inbox");
		const files = listFiles(dir).filter((f) => f.endsWith(".edi") || f.endsWith(".json"));
		const metas = files
			.filter((f) => !f.endsWith(".status.json"))
			.map((f) => nachrichtMeta(rolle, "inbox", f));
		res.json(metas);
	} catch (e) {
		badRequest(res, e);
	}
});

app.get(`${API}/rollen/:rolle/outbox`, (req: Request, res: Response) => {
	try {
		const rolle = param(req, "rolle");
		const dir = safeMarktPath(rolle, "outbox");
		const files = listFiles(dir).filter((f) => f.endsWith(".edi") || f.endsWith(".json"));
		const metas = files
			.filter((f) => !f.endsWith(".status.json"))
			.map((f) => nachrichtMeta(rolle, "outbox", f));
		res.json(metas);
	} catch (e) {
		badRequest(res, e);
	}
});

app.get(`${API}/rollen/:rolle/state`, (req: Request, res: Response) => {
	try {
		const rolle = param(req, "rolle");
		const state = readJsonSafe(safeMarktPath(rolle, "state.json"));
		res.json(state);
	} catch (e) {
		badRequest(res, e);
	}
});

app.get(`${API}/nachrichten/:rolle/:box/:datei`, (req: Request, res: Response) => {
	let filePath: string;
	let rolle: string;
	let box: string;
	let datei: string;
	try {
		rolle = param(req, "rolle");
		box = param(req, "box");
		datei = param(req, "datei");
		filePath = safeMarktPath(rolle, box, datei);
	} catch (e) {
		badRequest(res, e);
		return;
	}
	if (!existsSync(filePath)) {
		res.status(404).json({ error: "not found" });
		return;
	}
	const inhalt = readFileSync(filePath, "utf-8");
	const meta = nachrichtMeta(rolle, box, datei);

	let edifact: string | undefined;
	if (datei.endsWith(".json")) {
		const ediPath = filePath.replace(/\.json$/, ".edi");
		if (existsSync(ediPath)) edifact = readFileSync(ediPath, "utf-8");
	} else {
		edifact = inhalt;
	}

	res.json({ meta, inhalt, edifact });
});

app.get(`${API}/status`, (_req: Request, res: Response) => {
	try {
		const output = cli(["status", MARKT]);
		res.json({ ok: true, ausgabe: output });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post(`${API}/sende`, (req: Request, res: Response) => {
	const { von, datei, an } = req.body;
	if (!isSafeSegment(String(von)) || !isSafeSegment(String(an)) || !isSafeSegment(String(datei))) {
		badRequest(res, new Error("ungültige Parameter (von/an/datei)"));
		return;
	}
	try {
		cli(["sende", von, an, datei, "--markt", MARKT]);
		res.json({ ok: true });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post(`${API}/verarbeite`, (req: Request, res: Response) => {
	const { rolle, datei } = req.body;
	let filePath: string;
	try {
		filePath = safeMarktPath(String(rolle), "inbox", String(datei));
	} catch (e) {
		badRequest(res, e);
		return;
	}
	try {
		const ausgabe = cli(["verarbeite", filePath, "--markt", MARKT]);
		res.json({ ok: true, ausgabe });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post(`${API}/verarbeite-alle`, (req: Request, res: Response) => {
	const { rolle } = req.body;
	if (!isSafeSegment(String(rolle))) {
		badRequest(res, new Error("ungültige Rolle"));
		return;
	}
	try {
		const ausgabe = cli(["verarbeite-alle", rolle, "--markt", MARKT]);
		res.json({ ok: true, ausgabe });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post(`${API}/nachrichten/:rolle`, (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const payload = req.body;
	const typRaw = payload.typ ?? "nachricht";
	if (!isSafeSegment(String(typRaw))) {
		badRequest(res, new Error("ungültiger Nachrichtentyp"));
		return;
	}
	let outboxDir: string;
	try {
		outboxDir = safeMarktPath(rolle, "outbox");
	} catch (e) {
		badRequest(res, e);
		return;
	}
	const existing = listFiles(outboxDir);
	const seq = String(existing.length + 1).padStart(3, "0");
	const filename = `${seq}_${typRaw}.json`;
	const filePath = join(outboxDir, filename);

	writeFileSync(filePath, JSON.stringify(payload, null, "\t"), "utf-8");
	res.json({ ok: true, datei: filename });
});

// --- Kreuzvalidator (STROMDAO sidecar) ---

registerKreuzvalidatorRoutes(app, API);

// --- Verification routes ---

const REFERENZDATEN = resolve("../mako-verify/referenzdaten");

app.get(`${API}/verifiziere/:rolle/:box/:datei`, (req: Request, res: Response) => {
	let filePath: string;
	try {
		const rolle = param(req, "rolle");
		const box = param(req, "box");
		const datei = param(req, "datei");
		filePath = safeMarktPath(rolle, box, datei);
	} catch (e) {
		badRequest(res, e);
		return;
	}

	if (!existsSync(filePath)) {
		res.status(404).json({ error: "Datei nicht gefunden" });
		return;
	}

	try {
		const ausgabe = cli(["verifiziere", filePath, "--referenzdaten", REFERENZDATEN]);
		const ergebnis = JSON.parse(ausgabe);
		res.json(ergebnis);
	} catch (e) {
		res.status(500).json({ error: `Verifikation fehlgeschlagen: ${String(e)}` });
	}
});

// Optional batch root directories. Caller may pass a label that maps to one
// of these absolute roots; arbitrary filesystem paths are not accepted.
const BATCH_ROOTS: Record<string, string> = {
	markt: MARKT,
	simulation: resolve("../mako-sim/simulation/nachrichten"),
};

app.post(`${API}/verifiziere-batch`, (req: Request, res: Response) => {
	const { verzeichnis } = req.body ?? {};
	const key = typeof verzeichnis === "string" && verzeichnis.length > 0 ? verzeichnis : "markt";
	const dir = BATCH_ROOTS[key];
	if (!dir) {
		badRequest(
			res,
			new Error(
				`unbekanntes verzeichnis '${key}'. erlaubt: ${Object.keys(BATCH_ROOTS).join(", ")}`,
			),
		);
		return;
	}

	if (!existsSync(dir)) {
		res.status(404).json({ error: "Verzeichnis nicht gefunden" });
		return;
	}

	try {
		const ausgabe = cli(["verifiziere-batch", dir, "--referenzdaten", REFERENZDATEN]);
		const ergebnis = JSON.parse(ausgabe);
		res.json(ergebnis);
	} catch (e) {
		res.status(500).json({ error: `Batch-Verifikation fehlgeschlagen: ${String(e)}` });
	}
});

app.post(`${API}/verifiziere-schritt`, (req: Request, res: Response) => {
	const { rolle, datei } = req.body ?? {};
	if (!rolle || !datei) {
		res.status(400).json({ error: "rolle und datei erforderlich" });
		return;
	}

	let filePath: string;
	try {
		filePath = safeMarktPath(String(rolle), "inbox", String(datei));
	} catch (e) {
		badRequest(res, e);
		return;
	}
	if (!existsSync(filePath)) {
		res.status(404).json({ error: "Datei nicht gefunden" });
		return;
	}

	try {
		// Step 1: process the message
		const verarbeiteAusgabe = cli(["verarbeite", filePath, "--markt", MARKT]);

		// Step 2: verify the message
		let verifikation: unknown = null;
		let verifikationOk = true;
		try {
			const verifiziereAusgabe = cli(["verifiziere", filePath, "--referenzdaten", REFERENZDATEN]);
			verifikation = JSON.parse(verifiziereAusgabe);
		} catch (e) {
			verifikation = { fehler: String(e) };
			verifikationOk = false;
		}

		res.json({
			ok: verifikationOk,
			verarbeitung: verarbeiteAusgabe,
			verifikation,
		});
	} catch (e) {
		res.status(500).json({ error: `Verarbeitung fehlgeschlagen: ${String(e)}` });
	}
});

// --- SSE file watcher ---

const clients: Set<Response> = new Set();

app.get(`${API}/events`, (_req: Request, res: Response) => {
	res.writeHead(200, {
		"Content-Type": "text/event-stream",
		"Cache-Control": "no-cache",
		Connection: "keep-alive",
	});
	res.write('data: {"type":"connected"}\n\n');
	clients.add(res);
	_req.on("close", () => clients.delete(res));
});

function broadcast(data: unknown) {
	const msg = `data: ${JSON.stringify(data)}\n\n`;
	for (const client of clients) {
		client.write(msg);
	}
}

if (existsSync(MARKT)) {
	const watcher = watch(MARKT, {
		ignoreInitial: true,
		ignored: /(^|[/\\])\../,
	});
	watcher.on("all", (event, path) => {
		broadcast({ type: "fs", event, path });
	});
}

const PORT = Number(process.env.PORT) || 3001;
app.listen(PORT, () => {
	console.log(`mako-ui server on http://localhost:${PORT}`);
	console.log(`  markt: ${MARKT}`);
	console.log(`  cli:   ${CLI}`);
});

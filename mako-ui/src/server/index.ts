import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { watch } from "chokidar";
import cors from "cors";
import express, { type Request, type Response } from "express";
import { registerRoutes as registerKreuzvalidatorRoutes } from "./kreuzvalidator.ts";

const app = express();
app.use(cors());
app.use(express.json());

const MARKT = resolve(process.env["MAKO_MARKT_PATH"] ?? "markt");
const CLI = resolve(process.env["MAKO_CLI_PATH"] ?? "../target/debug/mako-cli");

function cli(args: string[]): string {
	return execFileSync(CLI, args, { encoding: "utf-8", timeout: 10_000 });
}

function readJsonSafe(path: string): unknown {
	if (!existsSync(path)) return {};
	return JSON.parse(readFileSync(path, "utf-8"));
}

function listFiles(dir: string): string[] {
	if (!existsSync(dir)) return [];
	return readdirSync(dir).filter((f) => !f.startsWith(".")).sort();
}

function param(req: Request, name: string): string {
	const v = req.params[name];
	if (Array.isArray(v)) return v[0];
	return v ?? "";
}

function nachrichtMeta(rolle: string, box: string, datei: string) {
	const filePath = join(MARKT, rolle, box, datei);
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

app.get("/api/rollen", (_req: Request, res: Response) => {
	const dirs = readdirSync(MARKT).filter((d) => {
		const p = join(MARKT, d);
		return statSync(p).isDirectory() && d !== "log";
	});
	const rollen = dirs.map((name) => ({
		name,
		mp_id: "",
		verzeichnis: name,
	}));
	res.json(rollen);
});

app.get("/api/rollen/:rolle/inbox", (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const dir = join(MARKT, rolle, "inbox");
	const files = listFiles(dir).filter((f) => f.endsWith(".edi") || f.endsWith(".json"));
	const metas = files
		.filter((f) => !f.endsWith(".status.json"))
		.map((f) => nachrichtMeta(rolle, "inbox", f));
	res.json(metas);
});

app.get("/api/rollen/:rolle/outbox", (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const dir = join(MARKT, rolle, "outbox");
	const files = listFiles(dir).filter((f) => f.endsWith(".edi") || f.endsWith(".json"));
	const metas = files
		.filter((f) => !f.endsWith(".status.json"))
		.map((f) => nachrichtMeta(rolle, "outbox", f));
	res.json(metas);
});

app.get("/api/rollen/:rolle/state", (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const state = readJsonSafe(join(MARKT, rolle, "state.json"));
	res.json(state);
});

app.get("/api/nachrichten/:rolle/:box/:datei", (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const box = param(req, "box");
	const datei = param(req, "datei");
	const filePath = join(MARKT, rolle, box, datei);
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

app.get("/api/status", (_req: Request, res: Response) => {
	try {
		const output = cli(["status", MARKT]);
		res.json({ ok: true, ausgabe: output });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post("/api/sende", (req: Request, res: Response) => {
	const { von, datei, an } = req.body;
	try {
		cli(["sende", von, an, datei, "--markt", MARKT]);
		res.json({ ok: true });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post("/api/verarbeite", (req: Request, res: Response) => {
	const { rolle, datei } = req.body;
	const filePath = join(MARKT, rolle, "inbox", datei);
	try {
		const ausgabe = cli(["verarbeite", filePath, "--markt", MARKT]);
		res.json({ ok: true, ausgabe });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post("/api/verarbeite-alle", (req: Request, res: Response) => {
	const { rolle } = req.body;
	try {
		const ausgabe = cli(["verarbeite-alle", rolle, "--markt", MARKT]);
		res.json({ ok: true, ausgabe });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post("/api/nachrichten/:rolle", (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const payload = req.body;
	const outboxDir = join(MARKT, rolle, "outbox");
	const existing = listFiles(outboxDir);
	const seq = String(existing.length + 1).padStart(3, "0");
	const typ = payload.typ ?? "nachricht";
	const filename = `${seq}_${typ}.json`;
	const filePath = join(outboxDir, filename);

	writeFileSync(filePath, JSON.stringify(payload, null, "\t"), "utf-8");
	res.json({ ok: true, datei: filename });
});

// --- Kreuzvalidator (STROMDAO sidecar) ---

registerKreuzvalidatorRoutes(app);

// --- Verification routes ---

const REFERENZDATEN = resolve("../mako-verify/referenzdaten");

app.get("/api/verifiziere/:rolle/:box/:datei", (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const box = param(req, "box");
	const datei = param(req, "datei");
	const filePath = join(MARKT, rolle, box, datei);

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

app.post("/api/verifiziere-batch", (req: Request, res: Response) => {
	const { verzeichnis } = req.body ?? {};
	const dir = verzeichnis ? resolve(verzeichnis) : MARKT;

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

app.post("/api/verifiziere-schritt", (req: Request, res: Response) => {
	const { rolle, datei } = req.body ?? {};
	if (!rolle || !datei) {
		res.status(400).json({ error: "rolle und datei erforderlich" });
		return;
	}

	const filePath = join(MARKT, rolle, "inbox", datei);
	if (!existsSync(filePath)) {
		res.status(404).json({ error: "Datei nicht gefunden" });
		return;
	}

	try {
		// Step 1: process the message
		const verarbeiteAusgabe = cli(["verarbeite", filePath, "--markt", MARKT]);

		// Step 2: verify the message
		let verifikation: unknown = null;
		try {
			const verifiziereAusgabe = cli(["verifiziere", filePath, "--referenzdaten", REFERENZDATEN]);
			verifikation = JSON.parse(verifiziereAusgabe);
		} catch (e) {
			verifikation = { fehler: String(e) };
		}

		res.json({
			ok: true,
			verarbeitung: verarbeiteAusgabe,
			verifikation,
		});
	} catch (e) {
		res.status(500).json({ error: `Verarbeitung fehlgeschlagen: ${String(e)}` });
	}
});

// --- SSE file watcher ---

const clients: Set<Response> = new Set();

app.get("/api/events", (_req: Request, res: Response) => {
	res.writeHead(200, {
		"Content-Type": "text/event-stream",
		"Cache-Control": "no-cache",
		Connection: "keep-alive",
	});
	res.write("data: {\"type\":\"connected\"}\n\n");
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

const PORT = Number(process.env["PORT"]) || 3001;
app.listen(PORT, () => {
	console.log(`mako-ui server on http://localhost:${PORT}`);
	console.log(`  markt: ${MARKT}`);
	console.log(`  cli:   ${CLI}`);
});

import { useState } from "react";
import { Badge } from "@/components/ui/badge.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { api } from "@/lib/api.ts";
import { type ProzessDef, type ProzessSchrittDef, PROZESSE } from "@/lib/prozesse.ts";
import { rollenKuerzel, rollenLabel } from "@/lib/rollen.ts";

interface MessageFormProps {
	rolle: string;
	aktiverProzess: string | null;
	onSent: () => void;
}

export function MessageForm({ rolle, aktiverProzess, onSent }: MessageFormProps) {
	const prozess = PROZESSE.find((p) => p.key === aktiverProzess);
	const sendbareSchritte = prozess?.schritte.filter((s) => s.absender === rolle) ?? [];

	const [selectedSchritt, setSelectedSchritt] = useState<ProzessSchrittDef | null>(
		sendbareSchritte[0] ?? null,
	);
	const [maloId, setMaloId] = useState("51238696700");
	const [sending, setSending] = useState(false);
	const [lastResult, setLastResult] = useState<string | null>(null);

	if (!prozess) {
		return (
			<div className="p-4">
				<p className="text-muted-foreground text-sm">
					Prozess in der linken Spalte auswählen, um Nachrichten zu senden.
				</p>
			</div>
		);
	}

	if (sendbareSchritte.length === 0) {
		return (
			<div className="p-4">
				<p className="text-muted-foreground text-sm">
					Diese Rolle sendet in „{prozess.name}" keine Nachrichten.
				</p>
			</div>
		);
	}

	async function handleSend() {
		if (!selectedSchritt || !prozess) return;
		setSending(true);
		setLastResult(null);
		try {
			const payload = {
				typ: selectedSchritt.typ,
				malo_id: maloId,
				absender: rolle,
				empfaenger: selectedSchritt.empfaenger,
				absender_rolle: rollenKuerzel(rolle),
				empfaenger_rolle: rollenKuerzel(selectedSchritt.empfaenger),
				payload: {
					typ: selectedSchritt.typ,
					malo_id: maloId,
					sparte: prozess.kategorie.includes("Gas") ? "Gas" : "Strom",
				},
			};

			const result = await api.erstelle(rolle, payload);
			if (!result.ok) throw new Error("Erstellen fehlgeschlagen");

			await api.sende(rolle, result.datei, selectedSchritt.empfaenger);
			setLastResult(`Gesendet: ${result.datei} → ${rollenLabel(selectedSchritt.empfaenger)}`);
			onSent();
		} catch (e) {
			setLastResult(`Fehler: ${e}`);
		} finally {
			setSending(false);
		}
	}

	return (
		<div className="flex h-full flex-col overflow-auto p-4">
			<Card>
				<CardHeader className="pb-2">
					<CardTitle className="text-sm">
						Nachricht senden — {prozess.name}
					</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					{/* Schritt-Auswahl */}
					<div>
						<label className="mb-1 block text-xs text-muted-foreground">Schritt</label>
						<div className="flex flex-wrap gap-1.5">
							{sendbareSchritte.map((s) => (
								<button
									key={s.typ}
									type="button"
									className={`rounded border px-2 py-1 text-xs transition-colors ${
										selectedSchritt?.typ === s.typ
											? "border-primary bg-primary/10 font-medium"
											: "border-border hover:bg-accent"
									}`}
									onClick={() => setSelectedSchritt(s)}
								>
									{s.name}
								</button>
							))}
						</div>
					</div>

					{selectedSchritt && (
						<>
							{/* Empfänger */}
							<div className="flex items-center gap-2 text-xs">
								<Badge variant="outline">{selectedSchritt.nachrichtentyp}</Badge>
								<span className="text-muted-foreground">
									{rollenLabel(rolle)} → {rollenLabel(selectedSchritt.empfaenger)}
								</span>
							</div>

							<Separator />

							{/* Felder */}
							<div>
								<label className="mb-1 block text-xs text-muted-foreground" htmlFor="malo-id">
									MaLo-ID
								</label>
								<input
									id="malo-id"
									type="text"
									className="w-full rounded border border-input bg-background px-2 py-1.5 font-mono text-sm"
									value={maloId}
									onChange={(e) => setMaloId(e.target.value)}
								/>
							</div>

							<Button
								className="w-full"
								onClick={handleSend}
								disabled={sending}
							>
								{sending ? "Sende..." : `Senden → ${rollenKuerzel(selectedSchritt.empfaenger)}`}
							</Button>

							{lastResult && (
								<p className={`text-xs ${lastResult.startsWith("Fehler") ? "text-destructive" : "text-emerald-600"}`}>
									{lastResult}
								</p>
							)}
						</>
					)}
				</CardContent>
			</Card>

			{/* EdifactPreview inline */}
			{selectedSchritt && (
				<EdifactPreviewInline schritt={selectedSchritt} prozess={prozess} maloId={maloId} rolle={rolle} />
			)}
		</div>
	);
}

function EdifactPreviewInline({
	schritt,
	prozess,
	maloId,
	rolle,
}: {
	schritt: ProzessSchrittDef;
	prozess: ProzessDef;
	maloId: string;
	rolle: string;
}) {
	const [open, setOpen] = useState(false);

	const preview = generatePreviewEdifact(schritt, prozess, maloId, rolle);

	return (
		<div className="mt-3">
			<button
				type="button"
				className="text-xs text-primary underline-offset-2 hover:underline"
				onClick={() => setOpen((v) => !v)}
			>
				{open ? "EDIFACT-Vorschau ausblenden" : "EDIFACT-Vorschau anzeigen"}
			</button>
			{open && (
				<pre className="mt-1 max-h-48 overflow-auto rounded bg-muted p-3 font-mono text-[11px] leading-relaxed">
					{preview}
				</pre>
			)}
		</div>
	);
}

function generatePreviewEdifact(
	schritt: ProzessSchrittDef,
	_prozess: ProzessDef,
	maloId: string,
	rolle: string,
): string {
	const now = new Date();
	const datum = now.toISOString().slice(0, 10).replace(/-/g, "");
	const zeit = now.toISOString().slice(11, 15).replace(/:/g, "");
	const absender = rollenKuerzel(rolle);
	const empfaenger = rollenKuerzel(schritt.empfaenger);

	return [
		`UNB+UNOC:3+${absender}:500+${empfaenger}:500+${datum}:${zeit}+00001'`,
		`UNH+00001+${schritt.nachrichtentyp}:D:11A:UN:5.2e'`,
		`BGM+E01+${schritt.typ}+9'`,
		`DTM+137:${datum}:102'`,
		`NAD+MS+${absender}::293'`,
		`NAD+MR+${empfaenger}::293'`,
		`LOC+172+${maloId}'`,
		`UNT+7+00001'`,
		`UNZ+1+00001'`,
	].join("\n");
}

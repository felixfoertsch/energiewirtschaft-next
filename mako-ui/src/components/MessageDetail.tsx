import { useEffect, useState } from "react";
import { Badge } from "@/components/ui/badge.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { StatusBadge } from "@/components/StatusBadge.tsx";
import { rollenLabel } from "@/lib/rollen.ts";
import { api } from "@/lib/api.ts";
import type { NachrichtMeta } from "@/lib/types.ts";

interface MessageDetailProps {
	rolle: string;
	box: "inbox" | "outbox";
	datei: string;
	onRolleSwitch: (rolle: string) => void;
	onVerarbeitet?: () => void;
}

interface DetailData {
	meta: NachrichtMeta;
	inhalt: string;
	edifact?: string;
}

export function MessageDetail({ rolle, box, datei, onRolleSwitch, onVerarbeitet }: MessageDetailProps) {
	const [data, setData] = useState<DetailData | null>(null);
	const [showEdifact, setShowEdifact] = useState(false);
	const [showJson, setShowJson] = useState(false);
	const [verarbeitung, setVerarbeitung] = useState<string | null>(null);

	useEffect(() => {
		let cancelled = false;
		api.nachricht(rolle, box, datei).then((d) => {
			if (!cancelled) setData(d);
		});
		return () => {
			cancelled = true;
		};
	}, [rolle, box, datei]);

	if (!data) {
		return <p className="p-4 text-muted-foreground text-sm">Lade...</p>;
	}

	const { meta, inhalt, edifact } = data;

	return (
		<div className="flex h-full flex-col overflow-auto p-4">
			<Card>
				<CardHeader className="pb-2">
					<CardTitle className="flex items-center gap-2 text-base">
						<Badge variant="outline" className="font-mono">
							{meta.typ}
						</Badge>
						<span className="text-muted-foreground text-sm">{meta.datei}</span>
					</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					{/* Absender / Empfänger */}
					<div className="flex gap-4 text-sm">
						{meta.absender && (
							<span>
								Von:{" "}
								<button
									type="button"
									className="font-medium text-primary underline-offset-2 hover:underline"
									onClick={() => onRolleSwitch(meta.absender)}
								>
									{rollenLabel(meta.absender)}
								</button>
							</span>
						)}
						{meta.empfaenger && (
							<span>
								An:{" "}
								<button
									type="button"
									className="font-medium text-primary underline-offset-2 hover:underline"
									onClick={() => onRolleSwitch(meta.empfaenger)}
								>
									{rollenLabel(meta.empfaenger)}
								</button>
							</span>
						)}
					</div>

					{/* Status timeline */}
					<StatusBadge status={meta.status} />

					{/* Zeitpunkt */}
					{meta.zeitpunkt && (
						<p className="text-muted-foreground text-xs">
							{new Date(meta.zeitpunkt).toLocaleString("de-DE")}
						</p>
					)}

					{/* Verarbeiten button for inbox messages */}
					{box === "inbox" && !meta.status.verarbeitet && (
						<div>
							<Button
								size="sm"
								onClick={async () => {
									setVerarbeitung("läuft...");
									try {
										const result = await api.verarbeite(rolle, datei);
										setVerarbeitung(result.ok ? "Verarbeitet" : "Fehler");
										onVerarbeitet?.();
									} catch (e) {
										setVerarbeitung(`Fehler: ${e}`);
									}
								}}
								disabled={verarbeitung === "läuft..."}
							>
								{verarbeitung === "läuft..." ? "Verarbeite..." : "Verarbeiten"}
							</Button>
							{verarbeitung && verarbeitung !== "läuft..." && (
								<span className={`ml-2 text-xs ${verarbeitung === "Verarbeitet" ? "text-emerald-600" : "text-destructive"}`}>
									{verarbeitung}
								</span>
							)}
						</div>
					)}

					<Separator />

					{/* EDIFACT collapsible */}
					{edifact && (
						<div>
							<button
								type="button"
								className="mb-1 text-sm text-primary underline-offset-2 hover:underline"
								onClick={() => setShowEdifact((v) => !v)}
							>
								{showEdifact ? "EDIFACT ausblenden" : "EDIFACT anzeigen"}
							</button>
							{showEdifact && (
								<pre className="max-h-64 overflow-auto rounded bg-muted p-3 font-mono text-xs">
									{edifact}
								</pre>
							)}
						</div>
					)}

					{/* JSON collapsible */}
					<div>
						<button
							type="button"
							className="mb-1 text-sm text-primary underline-offset-2 hover:underline"
							onClick={() => setShowJson((v) => !v)}
						>
							{showJson ? "JSON ausblenden" : "JSON anzeigen"}
						</button>
						{showJson && (
							<pre className="max-h-64 overflow-auto rounded bg-muted p-3 font-mono text-xs">
								{tryFormatJson(inhalt)}
							</pre>
						)}
					</div>
				</CardContent>
			</Card>
		</div>
	);
}

function tryFormatJson(s: string): string {
	try {
		return JSON.stringify(JSON.parse(s), null, "\t");
	} catch {
		return s;
	}
}

import { useState } from "react";
import { cn } from "@/lib/utils.ts";
import { Badge } from "@/components/ui/badge.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import type { BatchErgebnis, Urteil, VerifikationsErgebnis } from "@/lib/types.ts";

interface BatchBerichtProps {
	ergebnis: BatchErgebnis;
	onClose: () => void;
}

type Filter = "alle" | "fehler" | "bestanden";

const urteilColor: Record<Urteil, string> = {
	Bestanden: "text-emerald-600 dark:text-emerald-400",
	Fehlgeschlagen: "text-destructive",
	NichtPruefbar: "text-yellow-600 dark:text-yellow-400",
};

const urteilBg: Record<Urteil, string> = {
	Bestanden: "bg-emerald-500/20",
	Fehlgeschlagen: "bg-destructive/20",
	NichtPruefbar: "bg-yellow-500/20",
};

function urteilLabel(u: Urteil | null): string {
	if (!u) return "–";
	switch (u) {
		case "Bestanden":
			return "✓";
		case "Fehlgeschlagen":
			return "✗";
		case "NichtPruefbar":
			return "○";
	}
}

function filterErgebnisse(ergebnisse: VerifikationsErgebnis[], filter: Filter): VerifikationsErgebnis[] {
	switch (filter) {
		case "alle":
			return ergebnisse;
		case "fehler":
			return ergebnisse.filter((e) => e.gesamt_urteil === "Fehlgeschlagen");
		case "bestanden":
			return ergebnisse.filter((e) => e.gesamt_urteil === "Bestanden");
	}
}

export function BatchBericht({ ergebnis, onClose }: BatchBerichtProps) {
	const [filter, setFilter] = useState<Filter>("alle");

	const filtered = filterErgebnisse(ergebnis.ergebnisse, filter);

	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm">
			<Card className="mx-4 flex max-h-[90vh] w-full max-w-4xl flex-col">
				<CardHeader className="pb-3">
					<div className="flex items-center justify-between">
						<CardTitle className="text-lg">Verifikationsbericht</CardTitle>
						<Button variant="outline" size="sm" onClick={onClose}>
							Schließen
						</Button>
					</div>

					{/* Summary stats */}
					<div className="flex gap-3 pt-2">
						<Badge variant="outline" className="gap-1">
							<span className="font-mono">{ergebnis.gesamt}</span> geprüft
						</Badge>
						<Badge variant="outline" className="gap-1 text-emerald-600 dark:text-emerald-400">
							<span className="font-mono">{ergebnis.bestanden}</span> bestanden
						</Badge>
						<Badge variant="outline" className="gap-1 text-destructive">
							<span className="font-mono">{ergebnis.fehlgeschlagen}</span> fehlgeschlagen
						</Badge>
						<Badge variant="outline" className="gap-1 text-yellow-600 dark:text-yellow-400">
							<span className="font-mono">{ergebnis.nicht_pruefbar}</span> nicht prüfbar
						</Badge>
					</div>

					<Separator className="mt-2" />

					{/* Filter buttons */}
					<div className="flex gap-2 pt-1">
						{(["alle", "fehler", "bestanden"] as const).map((f) => (
							<Button
								key={f}
								variant={filter === f ? "default" : "outline"}
								size="sm"
								onClick={() => setFilter(f)}
							>
								{f === "alle" ? "Alle" : f === "fehler" ? "Fehler" : "Bestanden"}
							</Button>
						))}
					</div>
				</CardHeader>

				<CardContent className="min-h-0 flex-1 overflow-hidden pb-4">
					<ScrollArea className="h-full">
						<table className="w-full text-sm">
							<thead>
								<tr className="border-b text-left text-muted-foreground text-xs">
									<th className="px-2 py-1.5">Datei</th>
									<th className="px-2 py-1.5">Typ</th>
									<th className="px-2 py-1.5">PI</th>
									<th className="px-2 py-1.5 text-center">AHB</th>
									<th className="px-2 py-1.5 text-center">EBD</th>
									<th className="px-2 py-1.5 text-center">Codec</th>
									<th className="px-2 py-1.5 text-center">Gesamt</th>
								</tr>
							</thead>
							<tbody>
								{filtered.map((e) => (
									<tr
										key={e.datei}
										className={cn(
											"border-b border-border/50",
											urteilBg[e.gesamt_urteil],
										)}
									>
										<td className="px-2 py-1 font-mono text-xs">{e.datei}</td>
										<td className="px-2 py-1">{e.nachrichtentyp}</td>
										<td className="px-2 py-1 font-mono text-xs">{e.pruefidentifikator ?? "–"}</td>
										<td className={cn("px-2 py-1 text-center", e.ahb ? urteilColor[e.ahb.urteil] : "text-muted-foreground")}>
											{urteilLabel(e.ahb?.urteil ?? null)}
										</td>
										<td className={cn("px-2 py-1 text-center", e.ebd ? urteilColor[e.ebd.urteil] : "text-muted-foreground")}>
											{urteilLabel(e.ebd?.urteil ?? null)}
										</td>
										<td className={cn("px-2 py-1 text-center", e.interop ? urteilColor[e.interop.urteil] : "text-muted-foreground")}>
											{urteilLabel(e.interop?.urteil ?? null)}
										</td>
										<td className={cn("px-2 py-1 text-center font-medium", urteilColor[e.gesamt_urteil])}>
											{urteilLabel(e.gesamt_urteil)}
										</td>
									</tr>
								))}
								{filtered.length === 0 && (
									<tr>
										<td colSpan={7} className="px-2 py-4 text-center text-muted-foreground text-sm">
											Keine Ergebnisse für diesen Filter.
										</td>
									</tr>
								)}
							</tbody>
						</table>
					</ScrollArea>
				</CardContent>
			</Card>
		</div>
	);
}

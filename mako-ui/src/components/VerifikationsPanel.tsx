import { EbdBaum } from "@/components/EbdBaum.tsx";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs.tsx";
import type {
	AhbErgebnis,
	AhbFeldErgebnis,
	EbdErgebnis,
	InteropErgebnis,
	Urteil,
	VerifikationsErgebnis,
} from "@/lib/types.ts";
import { cn } from "@/lib/utils.ts";

interface VerifikationsPanelProps {
	ergebnis: VerifikationsErgebnis;
	className?: string;
}

const urteilIcon: Record<Urteil, string> = {
	Bestanden: "✓",
	Fehlgeschlagen: "✗",
	NichtPruefbar: "○",
};

const urteilColor: Record<Urteil, string> = {
	Bestanden: "text-emerald-600 dark:text-emerald-400",
	Fehlgeschlagen: "text-destructive",
	NichtPruefbar: "text-yellow-600 dark:text-yellow-400",
};

function UrteilBadge({ urteil }: { urteil: Urteil }) {
	return <span className={cn("font-mono text-sm", urteilColor[urteil])}>{urteilIcon[urteil]}</span>;
}

// ---------------------------------------------------------------------------
// AHB Tab
// ---------------------------------------------------------------------------

function AhbTab({ ahb }: { ahb: AhbErgebnis | null }) {
	if (!ahb) {
		return (
			<p className="py-4 text-center text-muted-foreground text-sm">Keine AHB-Prüfung verfügbar.</p>
		);
	}

	return (
		<div className="space-y-2">
			<div className="flex items-center justify-between text-sm">
				<span>
					PI: <span className="font-mono">{ahb.pruefidentifikator}</span> — {ahb.nachrichtentyp}
				</span>
				<UrteilBadge urteil={ahb.urteil} />
			</div>
			{ahb.zusammenfassung && (
				<p className="text-muted-foreground text-xs">{ahb.zusammenfassung}</p>
			)}
			<ScrollArea className="max-h-64">
				<table className="w-full text-xs">
					<thead>
						<tr className="border-b text-left text-muted-foreground">
							<th className="px-1 py-1">Seg</th>
							<th className="px-1 py-1">Name</th>
							<th className="px-1 py-1">AHB-Ausdruck</th>
							<th className="px-1 py-1">Wert</th>
							<th className="px-1 py-1">Urteil</th>
						</tr>
					</thead>
					<tbody>
						{ahb.felder.map((f: AhbFeldErgebnis) => (
							<tr
								key={`${f.segment_code ?? "segment"}-${f.name}-${f.ahb_ausdruck}`}
								className={cn(
									"border-b border-border/50",
									f.urteil === "Fehlgeschlagen" && "bg-destructive/10",
								)}
							>
								<td className="px-1 py-0.5 font-mono">{f.segment_code ?? "–"}</td>
								<td className="px-1 py-0.5">{f.name}</td>
								<td className="px-1 py-0.5 font-mono">{f.ahb_ausdruck}</td>
								<td className="px-1 py-0.5 font-mono">
									{f.unser_wert ?? "–"}
									{f.erwarteter_wert && f.urteil === "Fehlgeschlagen" && (
										<span className="text-muted-foreground"> (erw: {f.erwarteter_wert})</span>
									)}
								</td>
								<td className="px-1 py-0.5 text-center">
									<UrteilBadge urteil={f.urteil} />
								</td>
							</tr>
						))}
					</tbody>
				</table>
			</ScrollArea>
		</div>
	);
}

// ---------------------------------------------------------------------------
// EBD Tab
// ---------------------------------------------------------------------------

function EbdTab({ ebd }: { ebd: EbdErgebnis | null }) {
	if (!ebd) {
		return (
			<p className="py-4 text-center text-muted-foreground text-sm">Keine EBD-Prüfung verfügbar.</p>
		);
	}

	return (
		<div className="space-y-3">
			<div className="flex items-center justify-between text-sm">
				<span>
					<span className="font-mono">{ebd.ebd_code}</span> — {ebd.ebd_name}
					{ebd.rolle && <span className="text-muted-foreground"> ({ebd.rolle})</span>}
				</span>
				<UrteilBadge urteil={ebd.urteil} />
			</div>
			{ebd.details && <p className="text-muted-foreground text-xs">{ebd.details}</p>}
			{ebd.unser_ergebnis && (
				<div className="rounded border bg-muted/50 p-2 text-xs">
					<span className="font-medium">Unser Ergebnis:</span> Schritt {ebd.unser_ergebnis.schritt}{" "}
					— {ebd.unser_ergebnis.beschreibung}
					{ebd.unser_ergebnis.antwortcode && (
						<span className="font-mono"> [{ebd.unser_ergebnis.antwortcode}]</span>
					)}
				</div>
			)}
			<EbdBaum
				ausgaenge={ebd.gueltige_ausgaenge}
				unserSchritt={ebd.unser_ergebnis?.schritt ?? null}
			/>
		</div>
	);
}

// ---------------------------------------------------------------------------
// Codec (Interop) Tab
// ---------------------------------------------------------------------------

function CodecTab({ interop }: { interop: InteropErgebnis | null }) {
	if (!interop) {
		return (
			<p className="py-4 text-center text-muted-foreground text-sm">
				Keine Codec-Prüfung verfügbar.
			</p>
		);
	}

	return (
		<div className="space-y-2">
			<div className="flex items-center justify-between text-sm">
				<div className="flex gap-3 text-xs">
					<span>
						Parse (unser):{" "}
						<span
							className={
								interop.parse_ok_unser
									? "text-emerald-600 dark:text-emerald-400"
									: "text-destructive"
							}
						>
							{interop.parse_ok_unser ? "OK" : "Fehler"}
						</span>
					</span>
					<span>
						Parse (Drittanbieter):{" "}
						<span
							className={
								interop.parse_ok_drittanbieter
									? "text-emerald-600 dark:text-emerald-400"
									: "text-destructive"
							}
						>
							{interop.parse_ok_drittanbieter ? "OK" : "Fehler"}
						</span>
					</span>
					<span>
						Roundtrip:{" "}
						<span
							className={
								interop.roundtrip_ok ? "text-emerald-600 dark:text-emerald-400" : "text-destructive"
							}
						>
							{interop.roundtrip_ok ? "OK" : "Fehler"}
						</span>
					</span>
				</div>
				<UrteilBadge urteil={interop.urteil} />
			</div>
			{interop.feldvergleiche.length > 0 && (
				<ScrollArea className="max-h-64">
					<table className="w-full text-xs">
						<thead>
							<tr className="border-b text-left text-muted-foreground">
								<th className="px-1 py-1">Feld</th>
								<th className="px-1 py-1">Unser Wert</th>
								<th className="px-1 py-1">Drittanbieter</th>
								<th className="px-1 py-1">Match</th>
							</tr>
						</thead>
						<tbody>
							{interop.feldvergleiche.map((fv) => (
								<tr
									key={`${fv.feld}-${fv.unser_wert ?? "leer"}-${fv.drittanbieter_wert ?? "leer"}`}
									className={cn(
										"border-b border-border/50",
										!fv.stimmt_ueberein && "bg-destructive/10",
									)}
								>
									<td className="px-1 py-0.5 font-mono">{fv.feld}</td>
									<td className="px-1 py-0.5 font-mono">{fv.unser_wert ?? "–"}</td>
									<td className="px-1 py-0.5 font-mono">{fv.drittanbieter_wert ?? "–"}</td>
									<td className="px-1 py-0.5 text-center">
										{fv.stimmt_ueberein ? (
											<span className="text-emerald-600 dark:text-emerald-400">✓</span>
										) : (
											<span className="text-destructive">✗</span>
										)}
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</ScrollArea>
			)}
		</div>
	);
}

// ---------------------------------------------------------------------------
// Main Panel
// ---------------------------------------------------------------------------

export function VerifikationsPanel({ ergebnis, className }: VerifikationsPanelProps) {
	return (
		<div className={cn("space-y-2", className)}>
			<div className="flex items-center justify-between">
				<h3 className="font-semibold text-sm">Verifikation</h3>
				<span className={cn("text-sm font-medium", urteilColor[ergebnis.gesamt_urteil])}>
					{ergebnis.gesamt_urteil}
				</span>
			</div>
			<Tabs defaultValue="ahb">
				<TabsList>
					<TabsTrigger value="ahb">AHB</TabsTrigger>
					<TabsTrigger value="ebd">EBD</TabsTrigger>
					<TabsTrigger value="codec">Codec</TabsTrigger>
				</TabsList>
				<TabsContent value="ahb">
					<AhbTab ahb={ergebnis.ahb} />
				</TabsContent>
				<TabsContent value="ebd">
					<EbdTab ebd={ergebnis.ebd} />
				</TabsContent>
				<TabsContent value="codec">
					<CodecTab interop={ergebnis.interop} />
				</TabsContent>
			</Tabs>
		</div>
	);
}

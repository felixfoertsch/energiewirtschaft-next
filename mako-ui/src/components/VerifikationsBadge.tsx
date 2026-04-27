import type { Urteil, VerifikationsErgebnis } from "@/lib/types.ts";
import { cn } from "@/lib/utils.ts";

interface VerifikationsBadgeProps {
	ergebnis: VerifikationsErgebnis;
	className?: string;
}

const urteilColor: Record<Urteil, string> = {
	Bestanden: "bg-emerald-500/20 text-emerald-700 dark:text-emerald-400",
	Fehlgeschlagen: "bg-destructive/20 text-destructive",
	NichtPruefbar: "bg-yellow-500/20 text-yellow-700 dark:text-yellow-400",
};

const nullColor = "bg-muted text-muted-foreground";

function urteilIcon(u: Urteil): string {
	switch (u) {
		case "Bestanden":
			return "✓";
		case "Fehlgeschlagen":
			return "✗";
		case "NichtPruefbar":
			return "○";
	}
}

interface Indicator {
	label: string;
	urteil: Urteil | null;
}

function deriveIndicators(e: VerifikationsErgebnis): Indicator[] {
	return [
		{ label: "AHB", urteil: e.ahb?.urteil ?? null },
		{ label: "EBD", urteil: e.ebd?.urteil ?? null },
		{ label: "Codec", urteil: e.interop?.urteil ?? null },
	];
}

export function VerifikationsBadge({ ergebnis, className }: VerifikationsBadgeProps) {
	const indicators = deriveIndicators(ergebnis);

	return (
		<div className={cn("inline-flex items-center gap-1", className)} title="AHB / EBD / Codec">
			{indicators.map((ind) => (
				<span
					key={ind.label}
					className={cn(
						"inline-flex items-center gap-0.5 rounded px-1 py-0.5 font-mono text-[10px] leading-none",
						ind.urteil ? urteilColor[ind.urteil] : nullColor,
					)}
					title={`${ind.label}: ${ind.urteil ?? "–"}`}
				>
					<span>{ind.urteil ? urteilIcon(ind.urteil) : "–"}</span>
					<span className="text-[9px]">{ind.label}</span>
				</span>
			))}
		</div>
	);
}

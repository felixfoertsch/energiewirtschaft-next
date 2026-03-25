import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { cn } from "@/lib/utils.ts";
import { KATEGORIEN, type ProzessDef, prozesseFuerRolle } from "@/lib/prozesse.ts";

interface ProzessListeProps {
	rolle: string;
	aktiverProzess: string | null;
	onSelect: (key: string) => void;
}

export function ProzessListe({ rolle, aktiverProzess, onSelect }: ProzessListeProps) {
	const verfuegbar = prozesseFuerRolle(rolle);

	if (verfuegbar.length === 0) {
		return (
			<div className="p-4">
				<p className="text-muted-foreground text-xs">Keine Prozesse für diese Rolle.</p>
			</div>
		);
	}

	const grouped = new Map<string, ProzessDef[]>();
	for (const kat of KATEGORIEN) {
		const procs = verfuegbar.filter((p) => p.kategorie === kat);
		if (procs.length > 0) grouped.set(kat, procs);
	}

	return (
		<ScrollArea className="h-full">
			<div className="space-y-4 p-4">
				{[...grouped.entries()].map(([kat, procs]) => (
					<div key={kat}>
						<h3 className="mb-1 font-semibold text-[11px] text-muted-foreground uppercase tracking-wider">
							{kat}
						</h3>
						<div className="space-y-0.5">
							{procs.map((p) => (
								<button
									key={p.key}
									type="button"
									className={cn(
										"w-full rounded px-2 py-1.5 text-left text-sm transition-colors hover:bg-accent",
										aktiverProzess === p.key && "bg-accent font-medium",
									)}
									onClick={() => onSelect(p.key)}
								>
									{p.name}
								</button>
							))}
						</div>
					</div>
				))}
			</div>
		</ScrollArea>
	);
}

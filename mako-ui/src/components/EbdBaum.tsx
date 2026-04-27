import type { EbdAusgang } from "@/lib/types.ts";
import { cn } from "@/lib/utils.ts";

interface EbdBaumProps {
	ausgaenge: EbdAusgang[];
	unserSchritt: string | null;
	className?: string;
}

export function EbdBaum({ ausgaenge, unserSchritt, className }: EbdBaumProps) {
	if (ausgaenge.length === 0) {
		return (
			<p className="py-2 text-center text-muted-foreground text-xs">
				Keine EBD-Schritte verfügbar.
			</p>
		);
	}

	return (
		<div className={cn("space-y-1", className)}>
			<h4 className="font-medium text-xs text-muted-foreground">Entscheidungsbaum</h4>
			<div className="relative space-y-0">
				{ausgaenge.map((a, i) => {
					const isMatch = unserSchritt !== null && a.schritt === unserSchritt;
					const isLast = i === ausgaenge.length - 1;

					return (
						<div
							key={`${a.schritt}-${a.antwortcode ?? "ohne-code"}-${a.beschreibung}`}
							className="flex gap-2"
						>
							{/* Vertical connector line */}
							<div className="flex w-5 flex-col items-center">
								<div
									className={cn(
										"flex h-5 w-5 shrink-0 items-center justify-center rounded-full text-[10px] font-mono",
										isMatch
											? "bg-emerald-500 text-white dark:bg-emerald-600"
											: "bg-muted text-muted-foreground",
									)}
								>
									{a.schritt.length <= 2 ? a.schritt : i + 1}
								</div>
								{!isLast && (
									<div className={cn("w-px flex-1", isMatch ? "bg-emerald-500/50" : "bg-border")} />
								)}
							</div>

							{/* Step content */}
							<div
								className={cn(
									"mb-1 flex-1 rounded border px-2 py-1.5 text-xs",
									isMatch
										? "border-emerald-500/50 bg-emerald-500/10 dark:border-emerald-600/50 dark:bg-emerald-900/20"
										: "border-border bg-background",
								)}
							>
								<div className="flex items-start justify-between gap-2">
									<span>{a.beschreibung}</span>
									{a.antwortcode && (
										<span className="shrink-0 rounded bg-muted px-1 font-mono text-[10px]">
											{a.antwortcode}
										</span>
									)}
								</div>
								{a.notiz && <p className="mt-0.5 text-muted-foreground text-[10px]">{a.notiz}</p>}
								{isMatch && (
									<span className="mt-0.5 block font-medium text-[10px] text-emerald-700 dark:text-emerald-400">
										← Unser Ergebnis
									</span>
								)}
							</div>
						</div>
					);
				})}
			</div>
		</div>
	);
}

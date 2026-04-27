import { Badge } from "@/components/ui/badge.tsx";
import { rollenLabel } from "@/lib/rollen.ts";
import { NACHRICHTEN_TYP_LABEL, type ProzessDef } from "@/lib/types.ts";

interface SchrittErklaerungProps {
	prozess?: ProzessDef;
	rolle: string;
}

export function SchrittErklaerung({ prozess, rolle }: SchrittErklaerungProps) {
	if (!prozess || !rolle) return null;

	const relevante = prozess.schritte.filter((s) => s.absender === rolle || s.empfaenger === rolle);
	if (relevante.length === 0) return null;

	return (
		<section className="rounded-md border border-border bg-card text-card-foreground">
			<header className="border-b border-border px-3 py-1.5">
				<h3 className="font-medium text-foreground text-xs">Deine Schritte in {prozess.name}</h3>
			</header>
			<ol className="divide-y divide-border">
				{relevante.map((s, i) => {
					const typLabel = NACHRICHTEN_TYP_LABEL[s.nachrichten_typ];
					return (
						<li
							key={`${s.typ}-${s.absender}-${s.empfaenger}-${s.name}`}
							className="flex gap-2 px-3 py-2"
						>
							<span className="mt-0.5 font-mono text-[10px] text-muted-foreground">{i + 1}</span>
							<div className="min-w-0 flex-1">
								<div className="flex items-baseline justify-between gap-2">
									<div className="flex min-w-0 items-baseline gap-1.5">
										<span className="font-medium text-foreground text-sm">{s.name}</span>
										<span className="truncate text-[10px] text-muted-foreground">
											{rollenLabel(s.absender)} → {rollenLabel(s.empfaenger)}
										</span>
									</div>
									{typLabel && (
										<Badge variant="outline" className="shrink-0 font-mono text-[10px]">
											{typLabel}
										</Badge>
									)}
								</div>
								<p className="mt-0.5 text-muted-foreground text-xs leading-relaxed">
									{s.erklaerung}
								</p>
							</div>
						</li>
					);
				})}
			</ol>
		</section>
	);
}

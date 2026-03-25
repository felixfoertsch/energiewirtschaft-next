import { cn } from "@/lib/utils.ts";
import { PROZESSE } from "@/lib/prozesse.ts";
import { rollenKuerzel } from "@/lib/rollen.ts";

interface ProcessTimelineProps {
	prozessKey: string | null;
	aktiveRolle: string;
}

export function ProcessTimeline({ prozessKey, aktiveRolle }: ProcessTimelineProps) {
	const prozess = PROZESSE.find((p) => p.key === prozessKey);
	if (!prozess) return null;

	return (
		<div className="border-t bg-muted/30 px-6 py-3">
			<div className="mb-1.5 text-[11px] text-muted-foreground">
				{prozess.kategorie} — {prozess.name}
			</div>
			<div className="flex items-center gap-1">
				{prozess.schritte.map((s, i) => {
					const istAbsender = s.absender === aktiveRolle;
					const istEmpfaenger = s.empfaenger === aktiveRolle;
					return (
						<div key={s.typ + i} className="flex items-center gap-1">
							{i > 0 && (
								<div className="h-px w-4 bg-border" />
							)}
							<div
								className={cn(
									"flex items-center gap-1 rounded-full border px-2.5 py-1 text-[11px]",
									istAbsender && "border-primary bg-primary/10 font-medium text-primary",
									istEmpfaenger && "border-accent-foreground/30 bg-accent",
									!istAbsender && !istEmpfaenger && "border-border text-muted-foreground",
								)}
								title={`${s.name}: ${rollenKuerzel(s.absender)} → ${rollenKuerzel(s.empfaenger)}`}
							>
								<span className="font-mono text-[10px] text-muted-foreground">
									{i + 1}
								</span>
								<span>{s.name}</span>
								<span className="text-[10px] text-muted-foreground">
									{rollenKuerzel(s.absender)}→{rollenKuerzel(s.empfaenger)}
								</span>
							</div>
						</div>
					);
				})}
			</div>
		</div>
	);
}

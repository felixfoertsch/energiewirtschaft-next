import { Badge } from "@/components/ui/badge.tsx";
import { Card, CardContent } from "@/components/ui/card.tsx";
import type { ProzessDef } from "@/lib/prozesse.ts";
import { rollenLabel } from "@/lib/rollen.ts";

export interface Aufgabe {
	prozessKey: string;
	prozessName: string;
	schrittName: string;
	zielRolle: string;
}

export function deriveAufgaben(
	aktiveRolle: string,
	states: Record<string, unknown>,
	prozesse: readonly ProzessDef[],
): Aufgabe[] {
	const aufgaben: Aufgabe[] = [];

	for (const [key, zustand] of Object.entries(states)) {
		const prozessKey = key.split("/")[0];
		const prozess = prozesse.find((p) => p.key === prozessKey);
		if (!prozess) continue;

		const zustandStr =
			typeof zustand === "string"
				? zustand
				: typeof zustand === "object" && zustand !== null
					? JSON.stringify(zustand)
					: String(zustand);

		// Find the next step that requires action from a different role
		for (const schritt of prozess.schritte) {
			if (schritt.absender !== aktiveRolle && schritt.empfaenger === aktiveRolle) {
				// This role needs to process incoming → could be a task
				// Only show if state suggests we're at this step
				if (
					zustandStr.includes("Eingegangen") ||
					zustandStr.includes("Empfangen") ||
					zustandStr.includes("Gesendet")
				) {
					aufgaben.push({
						prozessKey: key,
						prozessName: prozess.name,
						schrittName: schritt.name,
						zielRolle: schritt.absender,
					});
					break;
				}
			}
		}
	}

	return aufgaben;
}

interface AufgabenQueueProps {
	aufgaben: Aufgabe[];
	onRolleSwitch: (rolle: string) => void;
}

export function AufgabenQueue({ aufgaben, onRolleSwitch }: AufgabenQueueProps) {
	if (aufgaben.length === 0) return null;

	return (
		<div className="space-y-2 border-b p-4">
			<h3 className="flex items-center gap-2 font-semibold text-[11px] text-muted-foreground uppercase tracking-wider">
				Offene Aufgaben
				<Badge variant="secondary" className="text-[10px]">
					{aufgaben.length}
				</Badge>
			</h3>
			<div className="space-y-1.5">
				{aufgaben.map((a) => (
					<Card key={a.prozessKey} className="p-0">
						<CardContent className="flex items-center justify-between p-2">
							<div className="text-xs">
								<span className="font-medium">{a.prozessName}</span>
								<span className="text-muted-foreground"> — {a.schrittName}</span>
							</div>
							<button
								type="button"
								className="text-xs text-primary underline-offset-2 hover:underline"
								onClick={() => onRolleSwitch(a.zielRolle)}
							>
								→ {rollenLabel(a.zielRolle)}
							</button>
						</CardContent>
					</Card>
				))}
			</div>
		</div>
	);
}

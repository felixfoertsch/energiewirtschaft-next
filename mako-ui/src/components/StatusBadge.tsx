import type { NachrichtenStatus } from "@/lib/types.ts";
import { cn } from "@/lib/utils.ts";

interface StatusBadgeProps {
	status: NachrichtenStatus;
	className?: string;
}

interface Check {
	label: string;
	icon: string;
	variant: "gray" | "green" | "red";
}

function deriveChecks(status: NachrichtenStatus): Check[] {
	const checks: Check[] = [];

	if (status.erstellt) {
		checks.push({ label: "Erstellt", icon: "✓", variant: "gray" });
	}
	if (status.zugestellt) {
		checks.push({ label: "Zugestellt", icon: "✓✓", variant: "gray" });
	}
	if (status.contrl) {
		checks.push({
			label: "CONTRL",
			icon: "✓",
			variant: status.contrl.ergebnis === "positiv" ? "green" : "red",
		});
	}
	if (status.aperak) {
		checks.push({
			label: "APERAK",
			icon: "✓",
			variant: status.aperak.ergebnis === "positiv" ? "green" : "red",
		});
	}
	if (status.verarbeitet) {
		checks.push({ label: "Verarbeitet", icon: "✓✓✓", variant: "green" });
	}

	return checks;
}

const variantClasses = {
	gray: "text-muted-foreground",
	green: "text-emerald-600 dark:text-emerald-400",
	red: "text-destructive",
} as const;

export function StatusBadge({ status, className }: StatusBadgeProps) {
	const checks = deriveChecks(status);
	if (checks.length === 0) return null;

	return (
		<div className={cn("flex items-center gap-2 text-xs", className)}>
			{checks.map((c) => (
				<span
					key={c.label}
					className={cn("flex items-center gap-0.5", variantClasses[c.variant])}
					title={c.label}
				>
					<span className="font-mono text-[11px] leading-none">{c.icon}</span>
					<span className="text-[10px]">{c.label}</span>
				</span>
			))}
		</div>
	);
}

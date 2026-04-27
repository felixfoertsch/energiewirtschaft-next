import { CheckCheck, X } from "lucide-react";
import type { NachrichtenStatus } from "@/lib/types.ts";
import { cn } from "@/lib/utils.ts";

interface StatusBadgeProps {
	status: NachrichtenStatus;
	className?: string;
}

type IconKind = "text" | "ack-positiv" | "ack-negativ";

interface Check {
	label: string;
	icon: string;
	kind: IconKind;
	variant: "gray" | "green" | "red";
	title?: string;
}

function deriveChecks(status: NachrichtenStatus): Check[] {
	const checks: Check[] = [];

	if (status.erstellt) {
		checks.push({ label: "Erstellt", icon: "✓", kind: "text", variant: "gray" });
	}
	if (status.zugestellt) {
		checks.push({ label: "Zugestellt", icon: "✓✓", kind: "text", variant: "gray" });
	}
	if (status.contrl) {
		checks.push({
			label: "CONTRL",
			icon: "✓",
			kind: "text",
			variant: status.contrl.ergebnis === "positiv" ? "green" : "red",
		});
	}
	if (status.aperak) {
		checks.push({
			label: "APERAK",
			icon: "✓",
			kind: "text",
			variant: status.aperak.ergebnis === "positiv" ? "green" : "red",
		});
	}
	if (status.ack) {
		const positiv = status.ack.ergebnis === "positiv";
		checks.push({
			label: "ACK",
			icon: "",
			kind: positiv ? "ack-positiv" : "ack-negativ",
			variant: positiv ? "green" : "red",
			title: positiv
				? `Empfang bestätigt am ${status.ack.zeitpunkt}`
				: `Empfang abgelehnt am ${status.ack.zeitpunkt}`,
		});
	}
	if (status.verarbeitet) {
		checks.push({ label: "Verarbeitet", icon: "✓✓✓", kind: "text", variant: "green" });
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
					title={c.title ?? c.label}
				>
					{c.kind === "ack-positiv" ? (
						<CheckCheck className="h-3 w-3" aria-hidden="true" />
					) : c.kind === "ack-negativ" ? (
						<X className="h-3 w-3" aria-hidden="true" />
					) : (
						<span className="font-mono text-[11px] leading-none">{c.icon}</span>
					)}
					<span className="text-[10px]">{c.label}</span>
				</span>
			))}
		</div>
	);
}

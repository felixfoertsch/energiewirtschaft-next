import { ChevronDown, ChevronRight } from "lucide-react";
import { useEffect, useState } from "react";
import { Badge } from "@/components/ui/badge.tsx";
import { rollenKuerzel, rollenLabel } from "@/lib/rollen.ts";
import type { Rolle } from "@/lib/types.ts";

interface RollenSidebarProps {
	rollen: Rolle[];
	aktiveRolle: string;
	onRolleChange: (rolle: string) => void;
	unreadCounts?: Record<string, number>;
}

interface Gruppe {
	id: string;
	label: string;
	slugs: string[];
}

// Fixed visual ordering of role families. Slugs that the backend returns but
// that aren't listed here fall into a `weitere` bucket so nothing disappears
// when the role catalog grows.
const GRUPPEN: Gruppe[] = [
	{
		id: "lieferanten",
		label: "Lieferanten",
		slugs: ["lieferant_neu", "lieferant_alt", "lieferant_ersatz_grundversorgung"],
	},
	{
		id: "netzbetreiber",
		label: "Netzbetreiber",
		slugs: [
			"netzbetreiber",
			"netzbetreiber_alt",
			"netzbetreiber_neu",
			"anschlussnetzbetreiber",
			"anfordernder_netzbetreiber",
		],
	},
	{
		id: "messwesen",
		label: "Messwesen",
		slugs: [
			"messstellenbetreiber",
			"messstellenbetreiber_alt",
			"messstellenbetreiber_neu",
			"grundzustaendiger_messstellenbetreiber",
			"wettbewerblicher_messstellenbetreiber",
			"messdienstleister",
		],
	},
	{
		id: "bilanzierung",
		label: "Bilanzierung",
		slugs: ["bilanzkreisverantwortlicher", "bilanzkoordinator"],
	},
	{
		id: "erzeugung",
		label: "Erzeugung & Vermarktung",
		slugs: [
			"betreiber_erzeugungsanlage",
			"direktvermarkter",
			"aggregator",
			"ladepunktbetreiber",
		],
	},
	{
		id: "strom_sonstige",
		label: "Sonstige Strom",
		slugs: [
			"uebertragungsnetzbetreiber",
			"einsatzverantwortlicher",
			"betreiber_technische_ressource",
			"data_provider",
			"energieserviceanbieter",
			"registerbetreiber_hknr",
		],
	},
	{
		id: "gas",
		label: "Gas",
		slugs: [
			"fernleitungsnetzbetreiber",
			"marktgebietsverantwortlicher",
			"transportkunde",
			"kapazitaetsnutzer",
			"speicherstellenbetreiber",
			"einspeisenetzbetreiber",
			"ausspeisenetzbetreiber",
		],
	},
];

function gruppeFuerSlug(slug: string): string {
	for (const g of GRUPPEN) {
		if (g.slugs.includes(slug)) return g.id;
	}
	return "weitere";
}

function partition(rollen: Rolle[]): Map<string, Rolle[]> {
	const buckets = new Map<string, Rolle[]>();
	for (const g of GRUPPEN) buckets.set(g.id, []);
	buckets.set("weitere", []);

	const lookup = new Map(rollen.map((r) => [r.name, r] as const));
	for (const g of GRUPPEN) {
		for (const slug of g.slugs) {
			const r = lookup.get(slug);
			if (r) buckets.get(g.id)?.push(r);
		}
	}
	const known = new Set(GRUPPEN.flatMap((g) => g.slugs));
	for (const r of rollen) {
		if (!known.has(r.name)) buckets.get("weitere")?.push(r);
	}
	return buckets;
}

function gruppenSummen(buckets: Map<string, Rolle[]>, counts: Record<string, number>): Record<string, number> {
	const out: Record<string, number> = {};
	for (const [id, list] of buckets) {
		out[id] = list.reduce((sum, r) => sum + (counts[r.name] ?? 0), 0);
	}
	return out;
}

export function RollenSidebar({
	rollen,
	aktiveRolle,
	onRolleChange,
	unreadCounts = {},
}: RollenSidebarProps) {
	const buckets = partition(rollen);
	const sums = gruppenSummen(buckets, unreadCounts);
	const aktiveGruppe = gruppeFuerSlug(aktiveRolle);

	// Open by default: the group containing the active role plus any group that
	// has unread inboxes. Other groups stay collapsed to keep the chrome small.
	const initialOpen = new Set<string>([aktiveGruppe]);
	for (const [id, count] of Object.entries(sums)) {
		if (count > 0) initialOpen.add(id);
	}
	const [open, setOpen] = useState<Set<string>>(initialOpen);

	// When the active role moves into a previously-collapsed group (e.g. via the
	// AufgabenQueue), open that group automatically.
	useEffect(() => {
		setOpen((prev) => {
			if (prev.has(aktiveGruppe)) return prev;
			const next = new Set(prev);
			next.add(aktiveGruppe);
			return next;
		});
	}, [aktiveGruppe]);

	function toggle(id: string) {
		setOpen((prev) => {
			const next = new Set(prev);
			if (next.has(id)) next.delete(id);
			else next.add(id);
			return next;
		});
	}

	const allGruppen: { id: string; label: string }[] = [
		...GRUPPEN.map((g) => ({ id: g.id, label: g.label })),
		{ id: "weitere", label: "Weitere" },
	];

	return (
		<aside className="flex w-56 shrink-0 flex-col overflow-hidden border-r bg-muted/20">
			<div className="border-b px-3 py-2">
				<div className="font-medium text-muted-foreground text-xs uppercase tracking-wide">
					Rollen
				</div>
			</div>
			<nav className="flex-1 overflow-y-auto px-1 py-1">
				{allGruppen.map(({ id, label }) => {
					const list = buckets.get(id) ?? [];
					if (list.length === 0) return null;
					const isOpen = open.has(id);
					const sum = sums[id] ?? 0;
					return (
						<div key={id} className="mb-0.5">
							<button
								type="button"
								onClick={() => toggle(id)}
								className="flex w-full items-center gap-1 rounded px-2 py-1 text-left text-xs hover:bg-muted/60"
							>
								{isOpen ? (
									<ChevronDown className="h-3 w-3 shrink-0 text-muted-foreground" />
								) : (
									<ChevronRight className="h-3 w-3 shrink-0 text-muted-foreground" />
								)}
								<span className="flex-1 font-medium uppercase tracking-wide text-muted-foreground">
									{label}
								</span>
								<span className="text-[10px] text-muted-foreground/70">
									{list.length}
								</span>
								{sum > 0 && (
									<Badge variant="secondary" className="ml-1 h-4 min-w-4 px-1 text-[9px]">
										{sum}
									</Badge>
								)}
							</button>
							{isOpen && (
								<ul className="mt-0.5">
									{list.map((r) => {
										const active = r.name === aktiveRolle;
										const count = unreadCounts[r.name] ?? 0;
										return (
											<li key={r.name}>
												<button
													type="button"
													onClick={() => onRolleChange(r.name)}
													className={`flex w-full items-center gap-2 rounded px-2 py-1 pl-6 text-left text-xs ${
														active
															? "bg-primary/10 font-medium text-primary"
															: "hover:bg-muted/60"
													}`}
												>
													<span className="flex-1 truncate">{rollenLabel(r.name)}</span>
													<span
														className={`shrink-0 text-[9px] tabular-nums ${
															active ? "text-primary/70" : "text-muted-foreground/60"
														}`}
													>
														{rollenKuerzel(r.name)}
													</span>
													{count > 0 && (
														<Badge
															variant={active ? "default" : "secondary"}
															className="h-4 min-w-4 px-1 text-[9px]"
														>
															{count}
														</Badge>
													)}
												</button>
											</li>
										);
									})}
								</ul>
							)}
						</div>
					);
				})}
			</nav>
		</aside>
	);
}

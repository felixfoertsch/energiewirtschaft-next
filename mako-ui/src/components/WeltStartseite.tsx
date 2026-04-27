import { Badge } from "@/components/ui/badge.tsx";
import { ROLLEN } from "@/lib/rollen.ts";
import { cn } from "@/lib/utils.ts";
import {
	type MaLo,
	PERSONAS,
	type Persona,
	personaForRolle,
	WELT_BESCHREIBUNG,
	WELT_NAME,
} from "@/lib/welt.ts";

interface WeltStartseiteProps {
	malos: MaLo[];
	prozesseCount: number;
	onRolleSelect: (rolle: string) => void;
}

function personasFuerMalo(malo: MaLo): Persona[] {
	return Object.keys(malo.beziehungen)
		.map((rolle) => personaForRolle(rolle))
		.filter((persona): persona is Persona => Boolean(persona));
}

function initialen(persona: Persona): string {
	return `${persona.vorname[0] ?? ""}${persona.nachname[0] ?? ""}`.toUpperCase();
}

function sparteBadgeClass(sparte: MaLo["sparte"]): string {
	if (sparte === "strom")
		return "border-blue-500/30 bg-blue-500/10 text-blue-700 dark:text-blue-300";
	return "border-amber-500/30 bg-amber-500/10 text-amber-700 dark:text-amber-300";
}

function Beziehungsbild({ malo, personas }: { malo: MaLo; personas: Persona[] }) {
	const cx = 170;
	const cy = 70;
	const radiusX = 126;
	const radiusY = 46;
	const visiblePersonas = personas.slice(0, 12);
	const nodes = visiblePersonas.map((persona, index) => {
		const angle = (index / Math.max(visiblePersonas.length, 1)) * Math.PI * 2 - Math.PI / 2;
		return {
			persona,
			x: cx + Math.cos(angle) * radiusX,
			y: cy + Math.sin(angle) * radiusY,
		};
	});

	return (
		<div className="rounded border bg-muted/20 p-2">
			<svg
				role="img"
				aria-label={`Beziehungsbild für ${malo.bezeichnung}`}
				viewBox="0 0 340 140"
				className="h-32 w-full text-muted-foreground"
			>
				<title>{`Beziehungsbild für ${malo.bezeichnung}`}</title>
				{nodes.map(({ persona, x, y }) => (
					<line
						key={`line-${persona.rolle}`}
						x1={cx}
						y1={cy}
						x2={x}
						y2={y}
						className="stroke-border"
						strokeWidth="1"
					/>
				))}
				<rect
					x={cx - 42}
					y={cy - 16}
					width="84"
					height="32"
					rx="7"
					className="fill-card stroke-border"
					strokeWidth="1"
				/>
				<text
					x={cx}
					y={cy - 2}
					textAnchor="middle"
					className="fill-foreground font-medium text-[10px]"
				>
					MaLo
				</text>
				<text
					x={cx}
					y={cy + 11}
					textAnchor="middle"
					className="fill-muted-foreground font-mono text-[8px]"
				>
					{malo.id}
				</text>
				{nodes.map(({ persona, x, y }) => (
					<g key={persona.rolle}>
						<circle
							cx={x}
							cy={y}
							r="14"
							className="fill-background stroke-border"
							strokeWidth="1"
						/>
						<text
							x={x}
							y={y + 3}
							textAnchor="middle"
							className="fill-foreground font-semibold text-[8px]"
						>
							{persona.rollenKuerzel.slice(0, 5)}
						</text>
					</g>
				))}
			</svg>
		</div>
	);
}

function PersonaChip({
	persona,
	onSelect,
}: {
	persona: Persona;
	onSelect: (rolle: string) => void;
}) {
	const farbe = ROLLEN[persona.rolle]?.farbe ?? "bg-muted";

	return (
		<button
			type="button"
			className="inline-flex max-w-full items-center gap-1.5 rounded border bg-background px-2 py-1 text-left text-[11px] transition-colors hover:bg-muted/60"
			onClick={() => onSelect(persona.rolle)}
			title={`${persona.vorname} ${persona.nachname} · ${persona.firma}`}
		>
			<span
				className={cn(
					"inline-flex h-6 w-6 shrink-0 items-center justify-center rounded-full font-semibold text-[9px] text-white",
					farbe,
				)}
			>
				{initialen(persona)}
			</span>
			<span className="min-w-0">
				<span className="block truncate font-medium leading-3">
					{persona.vorname} {persona.nachname}
				</span>
				<span className="block truncate text-muted-foreground leading-3">
					{persona.rollenKuerzel}
				</span>
			</span>
		</button>
	);
}

function StoryKarte({
	malo,
	onRolleSelect,
}: {
	malo: MaLo;
	onRolleSelect: (rolle: string) => void;
}) {
	const personas = personasFuerMalo(malo);

	return (
		<article className="space-y-3 rounded border bg-card p-4">
			<div className="flex items-start justify-between gap-3">
				<div className="min-w-0">
					<h3 className="truncate font-semibold text-sm">{malo.bezeichnung}</h3>
					<p className="font-mono text-[11px] text-muted-foreground">{malo.id}</p>
				</div>
				<Badge
					variant="outline"
					className={cn("shrink-0 capitalize", sparteBadgeClass(malo.sparte))}
				>
					{malo.sparte}
				</Badge>
			</div>
			<p className="text-muted-foreground text-sm leading-5">{malo.story}</p>
			<Beziehungsbild malo={malo} personas={personas} />
			<div className="flex flex-wrap gap-1.5">
				{personas.map((persona) => (
					<PersonaChip key={persona.rolle} persona={persona} onSelect={onRolleSelect} />
				))}
			</div>
		</article>
	);
}

export function WeltStartseite({ malos, prozesseCount, onRolleSelect }: WeltStartseiteProps) {
	return (
		<main className="min-h-0 flex-1 overflow-y-auto bg-background">
			<div className="mx-auto max-w-7xl space-y-5 p-5">
				<section className="space-y-3 rounded border bg-card p-5">
					<div>
						<p className="font-medium text-muted-foreground text-xs uppercase tracking-wide">
							Welt
						</p>
						<h2 className="mt-1 font-bold text-3xl tracking-tight">Welt: {WELT_NAME}</h2>
					</div>
					<p className="max-w-4xl text-muted-foreground text-sm leading-6">{WELT_BESCHREIBUNG}</p>
					<div className="flex flex-wrap gap-2 text-xs">
						<span className="rounded border bg-muted/30 px-2 py-1">
							{PERSONAS.length} Marktteilnehmer
						</span>
						<span className="rounded border bg-muted/30 px-2 py-1">{malos.length} Geschichten</span>
						<span className="rounded border bg-muted/30 px-2 py-1">{prozesseCount} Prozesse</span>
						<span className="rounded border bg-muted/30 px-2 py-1">2 Sparten (Strom + Gas)</span>
					</div>
				</section>

				<section className="grid gap-3 md:grid-cols-2">
					{malos.map((malo) => (
						<StoryKarte key={malo.id} malo={malo} onRolleSelect={onRolleSelect} />
					))}
				</section>

				<p className="rounded border bg-muted/20 px-3 py-2 text-muted-foreground text-xs">
					Wähle einen Marktteilnehmer in der Sidebar oder klicke direkt auf eine Person, um in seine
					Rolle zu schlüpfen.
				</p>
			</div>
		</main>
	);
}

export interface RollenDefinition {
	name: string;
	label: string;
	kuerzel: string;
	farbe: string;
}

export const ROLLEN: Record<string, RollenDefinition> = {
	lieferant_neu: {
		name: "lieferant_neu",
		label: "Lieferant Neu",
		kuerzel: "LFN",
		farbe: "bg-blue-500",
	},
	netzbetreiber: {
		name: "netzbetreiber",
		label: "Netzbetreiber",
		kuerzel: "NB",
		farbe: "bg-emerald-500",
	},
	lieferant_alt: {
		name: "lieferant_alt",
		label: "Lieferant Alt",
		kuerzel: "LFA",
		farbe: "bg-orange-500",
	},
	messstellenbetreiber: {
		name: "messstellenbetreiber",
		label: "Messstellenbetreiber",
		kuerzel: "MSB",
		farbe: "bg-purple-500",
	},
	bilanzkreisverantwortlicher: {
		name: "bilanzkreisverantwortlicher",
		label: "Bilanzkreisverantwortlicher",
		kuerzel: "BKV",
		farbe: "bg-rose-500",
	},
	marktgebietsverantwortlicher: {
		name: "marktgebietsverantwortlicher",
		label: "Marktgebietsverantwortlicher",
		kuerzel: "MGV",
		farbe: "bg-amber-500",
	},
};

export function rollenLabel(name: string): string {
	return ROLLEN[name]?.label ?? name;
}

export function rollenKuerzel(name: string): string {
	return ROLLEN[name]?.kuerzel ?? name;
}

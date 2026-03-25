import { useCallback, useEffect, useState } from "react";
import { Badge } from "@/components/ui/badge.tsx";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs.tsx";
import { api, subscribeEvents } from "@/lib/api.ts";
import type { NachrichtMeta, Rolle } from "@/lib/types.ts";

const ROLLEN_LABELS: Record<string, string> = {
	lieferant_neu: "Lieferant Neu",
	netzbetreiber: "Netzbetreiber",
	lieferant_alt: "Lieferant Alt",
	messstellenbetreiber: "MSB",
	bilanzkreisverantwortlicher: "BKV",
	marktgebietsverantwortlicher: "MGV",
};

function rollenLabel(name: string): string {
	return ROLLEN_LABELS[name] ?? name;
}

export function App() {
	const [rollen, setRollen] = useState<Rolle[]>([]);
	const [aktiveRolle, setAktiveRolle] = useState("");
	const [inbox, setInbox] = useState<NachrichtMeta[]>([]);
	const [outbox, setOutbox] = useState<NachrichtMeta[]>([]);

	const loadRollen = useCallback(async () => {
		try {
			const r = await api.rollen();
			setRollen(r);
			if (r.length > 0 && !aktiveRolle) setAktiveRolle(r[0].name);
		} catch {
			/* server not ready */
		}
	}, [aktiveRolle]);

	const loadMessages = useCallback(async () => {
		if (!aktiveRolle) return;
		try {
			const [i, o] = await Promise.all([
				api.inbox(aktiveRolle),
				api.outbox(aktiveRolle),
			]);
			setInbox(i);
			setOutbox(o);
		} catch {
			/* server not ready */
		}
	}, [aktiveRolle]);

	useEffect(() => {
		loadRollen();
	}, [loadRollen]);

	useEffect(() => {
		loadMessages();
	}, [loadMessages]);

	useEffect(() => {
		return subscribeEvents(() => {
			loadMessages();
		});
	}, [loadMessages]);

	return (
		<div className="flex h-screen flex-col">
			<header className="border-b px-6 py-3">
				<h1 className="font-bold text-xl">MaKo-Simulator</h1>
			</header>

			<Tabs
				value={aktiveRolle}
				onValueChange={setAktiveRolle}
				className="flex flex-1 flex-col overflow-hidden"
			>
				<div className="border-b px-6">
					<TabsList className="h-auto gap-2 bg-transparent p-0">
						{rollen.map((r) => (
							<TabsTrigger
								key={r.name}
								value={r.name}
								className="rounded-none border-b-2 border-transparent px-3 py-2 data-[state=active]:border-primary data-[state=active]:bg-transparent"
							>
								{rollenLabel(r.name)}
							</TabsTrigger>
						))}
					</TabsList>
				</div>

				{rollen.map((r) => (
					<TabsContent
						key={r.name}
						value={r.name}
						className="mt-0 flex-1 overflow-hidden"
					>
						<div className="grid h-full grid-cols-3 gap-0">
							{/* Left: Prozesse (placeholder) */}
							<div className="border-r p-4">
								<h2 className="mb-2 font-semibold text-sm text-muted-foreground">
									Prozesse
								</h2>
								<p className="text-muted-foreground text-xs">
									Kommunikationslinien erscheinen hier.
								</p>
							</div>

							{/* Center: Inbox/Outbox */}
							<div className="border-r p-4">
								<h2 className="mb-2 font-semibold text-sm text-muted-foreground">
									Inbox ({inbox.length})
								</h2>
								<ScrollArea className="h-[40vh]">
									<MessageCards nachrichten={inbox} />
								</ScrollArea>
								<Separator className="my-4" />
								<h2 className="mb-2 font-semibold text-sm text-muted-foreground">
									Outbox ({outbox.length})
								</h2>
								<ScrollArea className="h-[40vh]">
									<MessageCards nachrichten={outbox} />
								</ScrollArea>
							</div>

							{/* Right: Detail / Form (placeholder) */}
							<div className="p-4">
								<h2 className="mb-2 font-semibold text-sm text-muted-foreground">
									Detail
								</h2>
								<p className="text-muted-foreground text-xs">
									Nachricht auswählen, um Details zu sehen.
								</p>
							</div>
						</div>
					</TabsContent>
				))}
			</Tabs>
		</div>
	);
}

function MessageCards({ nachrichten }: { nachrichten: NachrichtMeta[] }) {
	if (nachrichten.length === 0) {
		return <p className="text-muted-foreground text-xs">Keine Nachrichten.</p>;
	}
	return (
		<div className="space-y-2">
			{nachrichten.map((n) => (
				<Card key={n.datei} className="cursor-pointer hover:bg-accent/50">
					<CardHeader className="p-3 pb-1">
						<CardTitle className="flex items-center gap-2 text-sm">
							<Badge variant="outline" className="font-mono text-xs">
								{n.typ}
							</Badge>
							<span className="text-muted-foreground text-xs">{n.datei}</span>
						</CardTitle>
					</CardHeader>
					<CardContent className="p-3 pt-0">
						<StatusIndicator status={n.status} />
					</CardContent>
				</Card>
			))}
		</div>
	);
}

function StatusIndicator({ status }: { status: NachrichtMeta["status"] }) {
	const checks: string[] = [];
	if (status.erstellt) checks.push("Erstellt");
	if (status.zugestellt) checks.push("Zugestellt");
	if (status.contrl) {
		checks.push(
			status.contrl.ergebnis === "positiv" ? "CONTRL +" : "CONTRL -",
		);
	}
	if (status.aperak) {
		checks.push(
			status.aperak.ergebnis === "positiv" ? "APERAK +" : "APERAK -",
		);
	}
	if (status.verarbeitet) checks.push("Verarbeitet");

	if (checks.length === 0) return null;

	return (
		<div className="flex flex-wrap gap-1">
			{checks.map((c) => (
				<Badge
					key={c}
					variant={c.includes("-") ? "destructive" : "secondary"}
					className="text-[10px]"
				>
					{c}
				</Badge>
			))}
		</div>
	);
}

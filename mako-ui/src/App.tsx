import { useCallback, useEffect, useState } from "react";
import { AufgabenQueue, deriveAufgaben } from "@/components/AufgabenQueue.tsx";
import { MessageDetail } from "@/components/MessageDetail.tsx";
import { MessageForm } from "@/components/MessageForm.tsx";
import { MessageList } from "@/components/MessageList.tsx";
import { ProcessTimeline } from "@/components/ProcessTimeline.tsx";
import { ProzessListe } from "@/components/ProzessListe.tsx";
import { RollenTabs } from "@/components/RollenTabs.tsx";
import { api, subscribeEvents } from "@/lib/api.ts";
import type { NachrichtMeta, Rolle } from "@/lib/types.ts";

interface Selection {
	datei: string;
	box: "inbox" | "outbox";
}

export function App() {
	const [rollen, setRollen] = useState<Rolle[]>([]);
	const [aktiveRolle, setAktiveRolle] = useState("");
	const [inbox, setInbox] = useState<NachrichtMeta[]>([]);
	const [outbox, setOutbox] = useState<NachrichtMeta[]>([]);
	const [selection, setSelection] = useState<Selection | null>(null);
	const [unreadCounts, setUnreadCounts] = useState<Record<string, number>>({});
	const [aktiverProzess, setAktiverProzess] = useState<string | null>(null);
	const [rolleState, setRolleState] = useState<Record<string, unknown>>({});
	const [showForm, setShowForm] = useState(false);

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
			const [i, o, s] = await Promise.all([
				api.inbox(aktiveRolle),
				api.outbox(aktiveRolle),
				api.state(aktiveRolle),
			]);
			setInbox(i);
			setOutbox(o);
			setRolleState(s as Record<string, unknown>);
		} catch {
			/* server not ready */
		}
	}, [aktiveRolle]);

	const loadUnreadCounts = useCallback(async () => {
		if (rollen.length === 0) return;
		const counts: Record<string, number> = {};
		await Promise.all(
			rollen.map(async (r) => {
				try {
					const msgs = await api.inbox(r.name);
					counts[r.name] = msgs.filter((m) => !m.status.verarbeitet).length;
				} catch {
					counts[r.name] = 0;
				}
			}),
		);
		setUnreadCounts(counts);
	}, [rollen]);

	useEffect(() => {
		loadRollen();
	}, [loadRollen]);

	useEffect(() => {
		loadMessages();
	}, [loadMessages]);

	useEffect(() => {
		loadUnreadCounts();
	}, [loadUnreadCounts]);

	useEffect(() => {
		return subscribeEvents(() => {
			loadMessages();
			loadUnreadCounts();
		});
	}, [loadMessages, loadUnreadCounts]);

	const handleRolleChange = useCallback((rolle: string) => {
		setAktiveRolle(rolle);
		setSelection(null);
		setShowForm(false);
	}, []);

	const handleSelect = useCallback((datei: string, box: "inbox" | "outbox") => {
		setSelection({ datei, box });
		setShowForm(false);
	}, []);

	const handleProzessSelect = useCallback((key: string) => {
		setAktiverProzess((prev) => (prev === key ? null : key));
		setShowForm(true);
		setSelection(null);
	}, []);

	const aufgaben = deriveAufgaben(aktiveRolle, rolleState);

	return (
		<div className="flex h-screen flex-col">
			<header className="border-b px-6 py-3">
				<h1 className="font-bold text-xl">MaKo-Simulator</h1>
			</header>

			<RollenTabs
				rollen={rollen}
				aktiveRolle={aktiveRolle}
				onRolleChange={handleRolleChange}
				unreadCounts={unreadCounts}
			/>

			<div className="grid min-h-0 flex-1 grid-cols-[240px_1fr_1fr] overflow-hidden">
				{/* Left: Aufgaben + Prozesse */}
				<div className="flex flex-col overflow-hidden border-r">
					<AufgabenQueue aufgaben={aufgaben} onRolleSwitch={handleRolleChange} />
					<ProzessListe
						rolle={aktiveRolle}
						aktiverProzess={aktiverProzess}
						onSelect={handleProzessSelect}
					/>
				</div>

				{/* Center: Inbox/Outbox */}
				<div className="overflow-hidden border-r">
					<MessageList
						inbox={inbox}
						outbox={outbox}
						selectedDatei={selection?.datei ?? null}
						onSelect={handleSelect}
						onRolleSwitch={handleRolleChange}
					/>
				</div>

				{/* Right: Detail or Form */}
				<div className="overflow-hidden">
					{selection ? (
						<MessageDetail
							rolle={aktiveRolle}
							box={selection.box}
							datei={selection.datei}
							onRolleSwitch={handleRolleChange}
						/>
					) : showForm ? (
						<MessageForm
							rolle={aktiveRolle}
							aktiverProzess={aktiverProzess}
							onSent={loadMessages}
						/>
					) : (
						<div className="p-4">
							<p className="text-muted-foreground text-sm">
								Nachricht auswählen oder Prozess wählen, um zu senden.
							</p>
						</div>
					)}
				</div>
			</div>

			{/* Bottom: Process Timeline */}
			<ProcessTimeline prozessKey={aktiverProzess} aktiveRolle={aktiveRolle} />
		</div>
	);
}

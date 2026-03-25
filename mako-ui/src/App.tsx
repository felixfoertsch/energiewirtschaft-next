import { useCallback, useEffect, useState } from "react";
import { MessageDetail } from "@/components/MessageDetail.tsx";
import { MessageList } from "@/components/MessageList.tsx";
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

	const loadUnreadCounts = useCallback(async () => {
		if (rollen.length === 0) return;
		const counts: Record<string, number> = {};
		await Promise.all(
			rollen.map(async (r) => {
				try {
					const msgs = await api.inbox(r.name);
					const unread = msgs.filter((m) => !m.status.verarbeitet).length;
					counts[r.name] = unread;
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
	}, []);

	const handleSelect = useCallback((datei: string, box: "inbox" | "outbox") => {
		setSelection({ datei, box });
	}, []);

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

			<div className="grid min-h-0 flex-1 grid-cols-[240px_1fr_1fr]">
				{/* Left: Prozesse (placeholder for Task 9) */}
				<div className="border-r p-4">
					<h2 className="mb-2 font-semibold text-sm text-muted-foreground">
						Prozesse
					</h2>
					<p className="text-muted-foreground text-xs">
						Kommunikationslinien erscheinen hier.
					</p>
				</div>

				{/* Center: Inbox/Outbox */}
				<div className="border-r">
					<MessageList
						inbox={inbox}
						outbox={outbox}
						selectedDatei={selection?.datei ?? null}
						onSelect={handleSelect}
						onRolleSwitch={handleRolleChange}
					/>
				</div>

				{/* Right: Detail */}
				<div>
					{selection ? (
						<MessageDetail
							rolle={aktiveRolle}
							box={selection.box}
							datei={selection.datei}
							onRolleSwitch={handleRolleChange}
						/>
					) : (
						<div className="p-4">
							<p className="text-muted-foreground text-sm">
								Nachricht auswählen, um Details zu sehen.
							</p>
						</div>
					)}
				</div>
			</div>
		</div>
	);
}

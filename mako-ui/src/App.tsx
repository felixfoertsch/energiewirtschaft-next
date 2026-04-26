import { useCallback, useEffect, useState } from "react";
import { AufgabenQueue, deriveAufgaben } from "@/components/AufgabenQueue.tsx";
import { BatchBericht } from "@/components/BatchBericht.tsx";
import { MessageDetail } from "@/components/MessageDetail.tsx";
import { MessageForm } from "@/components/MessageForm.tsx";
import { MessageList } from "@/components/MessageList.tsx";
import { ProcessTimeline } from "@/components/ProcessTimeline.tsx";
import { ProzessListe } from "@/components/ProzessListe.tsx";
import { RollenSidebar } from "@/components/RollenSidebar.tsx";
import { Button } from "@/components/ui/button.tsx";
import { api, subscribeEvents } from "@/lib/api.ts";
import type { BatchErgebnis, NachrichtMeta, ProzessDef, Rolle } from "@/lib/types.ts";

interface Selection {
	datei: string;
	box: "inbox" | "outbox";
}

export function App() {
	const [rollen, setRollen] = useState<Rolle[]>([]);
	const [prozesse, setProzesse] = useState<ProzessDef[]>([]);
	const [aktiveRolle, setAktiveRolle] = useState("");
	const [inbox, setInbox] = useState<NachrichtMeta[]>([]);
	const [outbox, setOutbox] = useState<NachrichtMeta[]>([]);
	const [selection, setSelection] = useState<Selection | null>(null);
	const [unreadCounts, setUnreadCounts] = useState<Record<string, number>>({});
	const [aktiverProzess, setAktiverProzess] = useState<string | null>(null);
	const [rolleState, setRolleState] = useState<Record<string, unknown>>({});
	const [showForm, setShowForm] = useState(false);
	const [batchErgebnis, setBatchErgebnis] = useState<BatchErgebnis | null>(null);
	const [batchLoading, setBatchLoading] = useState(false);
	const [serverError, setServerError] = useState<string | null>(null);

	const loadRollen = useCallback(async () => {
		try {
			const r = await api.rollen();
			setServerError(null);
			setRollen(r);
			if (r.length > 0 && !aktiveRolle) setAktiveRolle(r[0].name);
		} catch (e) {
			setServerError(`Server nicht erreichbar (rollen): ${String(e)}`);
		}
	}, [aktiveRolle]);

	// One-shot: the engine's process catalog only changes on server restart,
	// so we never need to refetch this during the session.
	useEffect(() => {
		api.prozesse()
			.then(setProzesse)
			.catch((e) => setServerError(`Prozesskatalog nicht ladbar: ${String(e)}`));
	}, []);

	const loadMessages = useCallback(async () => {
		if (!aktiveRolle) return;
		try {
			const [i, o, s] = await Promise.all([
				api.inbox(aktiveRolle),
				api.outbox(aktiveRolle),
				api.state(aktiveRolle),
			]);
			setServerError(null);
			setInbox(i);
			setOutbox(o);
			setRolleState(s as Record<string, unknown>);
		} catch (e) {
			setServerError(`Nachrichten konnten nicht geladen werden: ${String(e)}`);
		}
	}, [aktiveRolle]);

	const loadUnreadCounts = useCallback(async () => {
		if (rollen.length === 0) return;
		const counts: Record<string, number> = {};
		const errors: string[] = [];
		await Promise.all(
			rollen.map(async (r) => {
				try {
					const msgs = await api.inbox(r.name);
					counts[r.name] = msgs.filter((m) => !m.status.verarbeitet).length;
				} catch (e) {
					counts[r.name] = 0;
					errors.push(`${r.name}: ${String(e)}`);
				}
			}),
		);
		setUnreadCounts(counts);
		if (errors.length > 0) {
			setServerError(`Fehler beim Lesen einzelner Inboxen: ${errors.join("; ")}`);
		}
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

	const aufgaben = deriveAufgaben(aktiveRolle, rolleState, prozesse);

	return (
		<div className="flex h-screen flex-col">
			<header className="flex items-center justify-between border-b px-6 py-3">
				<h1 className="font-bold text-xl">MaKo-Simulator</h1>
				<Button
					variant="outline"
					size="sm"
					disabled={batchLoading}
					onClick={async () => {
						setBatchLoading(true);
						try {
							const result = await api.verifiziereBatch();
							setBatchErgebnis(result);
						} catch (e) {
							setServerError(`Batch-Verifikation fehlgeschlagen: ${String(e)}`);
						} finally {
							setBatchLoading(false);
						}
					}}
				>
					{batchLoading ? "Verifiziere..." : "Simulation verifizieren"}
				</Button>
			</header>

			{serverError && (
				<div
					role="alert"
					className="flex items-center justify-between border-b border-destructive/30 bg-destructive/10 px-6 py-2 text-destructive text-xs"
				>
					<span>{serverError}</span>
					<button
						type="button"
						className="text-xs underline-offset-2 hover:underline"
						onClick={() => setServerError(null)}
					>
						schließen
					</button>
				</div>
			)}

			<div className="flex min-h-0 flex-1 overflow-hidden">
				<RollenSidebar
					rollen={rollen}
					aktiveRolle={aktiveRolle}
					onRolleChange={handleRolleChange}
					unreadCounts={unreadCounts}
				/>

				<div className="flex min-w-0 flex-1 flex-col overflow-hidden">
					<div className="grid min-h-0 flex-1 grid-cols-[240px_1fr_1fr] overflow-hidden">
						{/* Left: Aufgaben + Prozesse */}
						<div className="flex flex-col overflow-hidden border-r">
							<AufgabenQueue aufgaben={aufgaben} onRolleSwitch={handleRolleChange} />
							<ProzessListe
								rolle={aktiveRolle}
								aktiverProzess={aktiverProzess}
								onSelect={handleProzessSelect}
								prozesse={prozesse}
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
									onVerarbeitet={loadMessages}
								/>
							) : showForm ? (
								<MessageForm
									rolle={aktiveRolle}
									aktiverProzess={aktiverProzess}
									onSent={loadMessages}
									prozesse={prozesse}
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
					<ProcessTimeline prozessKey={aktiverProzess} aktiveRolle={aktiveRolle} prozesse={prozesse} />
				</div>
			</div>

			{/* Batch verification modal */}
			{batchErgebnis && (
				<BatchBericht
					ergebnis={batchErgebnis}
					onClose={() => setBatchErgebnis(null)}
				/>
			)}
		</div>
	);
}

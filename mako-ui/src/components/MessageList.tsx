import { Badge } from "@/components/ui/badge.tsx";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { StatusBadge } from "@/components/StatusBadge.tsx";
import { VerifikationsBadge } from "@/components/VerifikationsBadge.tsx";
import { rollenLabel } from "@/lib/rollen.ts";
import type { NachrichtMeta, VerifikationsErgebnis } from "@/lib/types.ts";

interface MessageListProps {
	inbox: NachrichtMeta[];
	outbox: NachrichtMeta[];
	selectedDatei: string | null;
	onSelect: (datei: string, box: "inbox" | "outbox") => void;
	onRolleSwitch: (rolle: string) => void;
	verifikationen?: Record<string, VerifikationsErgebnis>;
}

export function MessageList({
	inbox,
	outbox,
	selectedDatei,
	onSelect,
	onRolleSwitch,
	verifikationen,
}: MessageListProps) {
	return (
		<div className="flex h-full flex-col overflow-hidden p-4">
			<h2 className="mb-2 font-semibold text-sm text-muted-foreground">
				Inbox ({inbox.length})
			</h2>
			<ScrollArea className="min-h-0 flex-1">
				<MessageCards
					nachrichten={inbox}
					box="inbox"
					selectedDatei={selectedDatei}
					onSelect={onSelect}
					onRolleSwitch={onRolleSwitch}
					verifikationen={verifikationen}
				/>
			</ScrollArea>
			<Separator className="my-3" />
			<h2 className="mb-2 font-semibold text-sm text-muted-foreground">
				Outbox ({outbox.length})
			</h2>
			<ScrollArea className="min-h-0 flex-1">
				<MessageCards
					nachrichten={outbox}
					box="outbox"
					selectedDatei={selectedDatei}
					onSelect={onSelect}
					onRolleSwitch={onRolleSwitch}
					verifikationen={verifikationen}
				/>
			</ScrollArea>
		</div>
	);
}

interface MessageCardsProps {
	nachrichten: NachrichtMeta[];
	box: "inbox" | "outbox";
	selectedDatei: string | null;
	onSelect: (datei: string, box: "inbox" | "outbox") => void;
	onRolleSwitch: (rolle: string) => void;
	verifikationen?: Record<string, VerifikationsErgebnis>;
}

function MessageCards({
	nachrichten,
	box,
	selectedDatei,
	onSelect,
	onRolleSwitch,
	verifikationen,
}: MessageCardsProps) {
	if (nachrichten.length === 0) {
		return <p className="text-muted-foreground text-xs">Keine Nachrichten.</p>;
	}
	return (
		<div className="space-y-2">
			{nachrichten.map((n) => (
				<Card
					key={n.datei}
					className={`cursor-pointer transition-colors hover:bg-accent/50 ${
						selectedDatei === n.datei ? "border-primary bg-accent/30" : ""
					}`}
					onClick={() => onSelect(n.datei, box)}
				>
					<CardHeader className="p-3 pb-1">
						<CardTitle className="flex items-center gap-2 text-sm">
							<Badge variant="outline" className="font-mono text-xs">
								{n.typ}
							</Badge>
							<span className="text-muted-foreground text-xs">{n.datei}</span>
						</CardTitle>
					</CardHeader>
					<CardContent className="space-y-1.5 p-3 pt-0">
						<div className="flex gap-3 text-[11px] text-muted-foreground">
							{n.absender && (
								<span>
									Von:{" "}
									<button
										type="button"
										className="text-primary underline-offset-2 hover:underline"
										onClick={(e) => {
											e.stopPropagation();
											onRolleSwitch(n.absender);
										}}
									>
										{rollenLabel(n.absender)}
									</button>
								</span>
							)}
							{n.empfaenger && (
								<span>
									An:{" "}
									<button
										type="button"
										className="text-primary underline-offset-2 hover:underline"
										onClick={(e) => {
											e.stopPropagation();
											onRolleSwitch(n.empfaenger);
										}}
									>
										{rollenLabel(n.empfaenger)}
									</button>
								</span>
							)}
						</div>
						<div className="flex items-center gap-2">
							<StatusBadge status={n.status} />
							{verifikationen?.[n.datei] && (
								<VerifikationsBadge ergebnis={verifikationen[n.datei]} />
							)}
						</div>
					</CardContent>
				</Card>
			))}
		</div>
	);
}

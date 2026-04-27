import Form, { type IChangeEvent } from "@rjsf/core";
import type { RegistryWidgetsType, RJSFSchema, UiSchema, WidgetProps } from "@rjsf/utils";
import validator from "@rjsf/validator-ajv8";
import { useEffect, useMemo, useState } from "react";
import { Badge } from "@/components/ui/badge.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { api, type ErstelleValidiertAntwort } from "@/lib/api.ts";
import type { ProzessDef, SchrittDef } from "@/lib/prozesse.ts";
import { rollenKuerzel, rollenLabel } from "@/lib/rollen.ts";
import type { AhbFeldErgebnis } from "@/lib/types.ts";
import { NACHRICHTEN_TYP_LABEL } from "@/lib/types.ts";
import {
	buildContext,
	empfaengerKandidaten,
	MALOS,
	type MaLo,
	type Persona,
	seedFormDefaults,
} from "@/lib/welt.ts";

interface MessageFormProps {
	rolle: string;
	aktiverProzess: string | null;
	onSent: () => void;
	prozesse: readonly ProzessDef[];
}

type FormFields = Record<string, unknown>;

type SchemaState =
	| { status: "idle"; schema: null; error: null }
	| { status: "loading"; schema: null; error: null }
	| { status: "loaded"; schema: RJSFSchema; error: null }
	| { status: "error"; schema: null; error: string };

const uiSchema: UiSchema<FormFields> = {
	"ui:submitButtonOptions": {
		norender: true,
	},
};

const isoWidgets: RegistryWidgetsType<FormFields> = {
	DateWidget: IsoTextWidget("YYYY-MM-DD", "^\\d{4}-\\d{2}-\\d{2}$"),
	DateTimeWidget: IsoTextWidget("YYYY-MM-DD HH:MM", "^\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}$"),
	TimeWidget: IsoTextWidget("HH:MM", "^\\d{2}:\\d{2}$"),
};

export function MessageForm({ rolle, aktiverProzess, onSent, prozesse }: MessageFormProps) {
	const prozess = prozesse.find((p) => p.key === aktiverProzess);
	const sendbareSchritte = useMemo(
		() => prozess?.schritte.filter((s) => s.absender === rolle) ?? [],
		[prozess, rolle],
	);

	const [selectedTyp, setSelectedTyp] = useState<string | null>(sendbareSchritte[0]?.typ ?? null);
	const [selectedMaloId, setSelectedMaloId] = useState<string | null>(null);
	const [schemaState, setSchemaState] = useState<SchemaState>({
		status: "idle",
		schema: null,
		error: null,
	});
	const [schemaReload, setSchemaReload] = useState(0);
	const [empfaengerId, setEmpfaengerId] = useState("");
	const [formData, setFormData] = useState<FormFields>({});
	const [sending, setSending] = useState(false);
	const [lastResult, setLastResult] = useState<ErstelleValidiertAntwort | null>(null);

	const selectedSchritt =
		sendbareSchritte.find((schritt) => schritt.typ === selectedTyp) ?? sendbareSchritte[0] ?? null;

	const empfaengerKandidatenListe = useMemo(() => {
		if (!selectedSchritt) return [];
		return empfaengerKandidaten(rolle, selectedSchritt.empfaenger);
	}, [rolle, selectedSchritt]);

	const aktiverEmpfaenger =
		empfaengerKandidatenListe.find((persona) => persona.mpId === empfaengerId) ??
		empfaengerKandidatenListe[0] ??
		null;

	const maloKandidaten = useMemo(() => {
		if (!selectedSchritt) return [];
		return MALOS.filter(
			(malo) => malo.beziehungen[rolle] && malo.beziehungen[selectedSchritt.empfaenger],
		);
	}, [rolle, selectedSchritt]);

	const selectedMalo =
		maloKandidaten.find((malo) => malo.id === selectedMaloId) ?? maloKandidaten[0] ?? undefined;

	useEffect(() => {
		setSelectedTyp(sendbareSchritte[0]?.typ ?? null);
		setSelectedMaloId(null);
		setEmpfaengerId("");
		setFormData({});
		setLastResult(null);
	}, [sendbareSchritte]);

	useEffect(() => {
		void schemaReload;

		if (!selectedSchritt) {
			setSchemaState({ status: "idle", schema: null, error: null });
			return;
		}

		let cancelled = false;
		setSchemaState({ status: "loading", schema: null, error: null });
		setFormData({});
		setLastResult(null);

		api
			.schema(selectedSchritt.typ)
			.then((schema) => {
				if (cancelled) return;
				setSchemaState({ status: "loaded", schema, error: null });
			})
			.catch((error) => {
				if (cancelled) return;
				setSchemaState({
					status: "error",
					schema: null,
					error: `Schema konnte nicht geladen werden: ${String(error)}`,
				});
			});

		return () => {
			cancelled = true;
		};
	}, [selectedSchritt, schemaReload]);

	useEffect(() => {
		if (!selectedSchritt || schemaState.status !== "loaded" || !aktiverEmpfaenger) {
			return;
		}

		const context = buildContext(rolle, aktiverEmpfaenger, selectedMalo);
		if (!context) return;

		setFormData(seedFormDefaults(schemaState.schema, context));
	}, [
		rolle,
		selectedSchritt,
		selectedMalo,
		aktiverEmpfaenger,
		schemaState.status,
		schemaState.schema,
	]);

	if (!prozess) {
		return (
			<div className="p-4">
				<p className="text-muted-foreground text-sm">
					Prozess in der linken Spalte auswählen, um Nachrichten zu senden.
				</p>
			</div>
		);
	}

	if (sendbareSchritte.length === 0) {
		return (
			<div className="p-4">
				<p className="text-muted-foreground text-sm">
					Diese Rolle sendet in „{prozess.name}" keine Nachrichten.
				</p>
			</div>
		);
	}

	async function handleSubmit(event: IChangeEvent<FormFields>) {
		if (!selectedSchritt || schemaState.status !== "loaded" || !aktiverEmpfaenger) return;

		const fields = asFormFields(event.formData);
		setSending(true);
		setLastResult(null);

		try {
			const result = await api.erstelleValidiert({
				rolle,
				empfaenger: selectedSchritt.empfaenger,
				empfaenger_id: aktiverEmpfaenger.mpId,
				typ: selectedSchritt.typ,
				fields,
			});
			setLastResult(result);
			if (result.ok) onSent();
		} catch (error) {
			setLastResult({
				ok: false,
				wire_format: "",
				fehler: `Nachricht konnte nicht erstellt werden: ${String(error)}`,
			});
		} finally {
			setSending(false);
		}
	}

	function handleSchrittSelect(schritt: SchrittDef) {
		setSelectedTyp(schritt.typ);
		setSelectedMaloId(null);
		setEmpfaengerId("");
		setFormData({});
		setLastResult(null);
	}

	function handleEmpfaengerChange(mpId: string) {
		setEmpfaengerId(mpId);
		setLastResult(null);
	}

	function handleMaloChange(maloId: string) {
		setSelectedMaloId(maloId);
		setLastResult(null);
	}

	return (
		<div className="flex h-full flex-col overflow-auto p-4">
			<Card>
				<CardHeader className="pb-2">
					<CardTitle className="text-sm">Nachricht senden — {prozess.name}</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<div>
						<p className="mb-1 text-xs text-muted-foreground">Schritt</p>
						<div className="flex flex-wrap gap-1.5">
							{sendbareSchritte.map((s) => (
								<button
									key={`${s.typ}-${s.empfaenger}-${s.name}`}
									type="button"
									className={`rounded border px-2 py-1 text-xs transition-colors ${
										selectedSchritt?.typ === s.typ
											? "border-primary bg-primary/10 font-medium"
											: "border-border hover:bg-accent"
									}`}
									onClick={() => handleSchrittSelect(s)}
								>
									{s.name}
								</button>
							))}
						</div>
					</div>

					{selectedSchritt && (
						<>
							<div className="flex items-center gap-2 text-xs">
								<Badge variant="outline">
									{NACHRICHTEN_TYP_LABEL[selectedSchritt.nachrichten_typ]}
								</Badge>
								<span className="text-muted-foreground">
									{rollenLabel(rolle)} → {rollenLabel(selectedSchritt.empfaenger)}
								</span>
							</div>

							<Separator />

							<EmpfaengerAuswahl
								aktiverEmpfaenger={aktiverEmpfaenger}
								kandidaten={empfaengerKandidatenListe}
								schritt={selectedSchritt}
								onChange={handleEmpfaengerChange}
							/>

							{maloKandidaten.length > 0 && (
								<MaLoPicker
									malos={maloKandidaten}
									selectedMalo={selectedMalo}
									onChange={handleMaloChange}
								/>
							)}

							{schemaState.status === "loading" && <SchemaSkeleton />}
							{schemaState.status === "error" && (
								<div
									role="alert"
									className="rounded border border-destructive/30 bg-destructive/10 p-3 text-destructive text-xs"
								>
									<p>{schemaState.error}</p>
									<Button
										type="button"
										variant="outline"
										size="sm"
										className="mt-2 h-7"
										onClick={() => setSchemaReload((value) => value + 1)}
									>
										Retry
									</Button>
								</div>
							)}
							{schemaState.status === "loaded" && (
								<Form<FormFields>
									className="rjsf"
									formData={formData}
									schema={schemaState.schema}
									uiSchema={uiSchema}
									validator={validator}
									widgets={isoWidgets}
									onChange={(event) => setFormData(asFormFields(event.formData))}
									onSubmit={handleSubmit}
								>
									<Button
										className="mt-3 w-full"
										disabled={sending || !aktiverEmpfaenger}
										type="submit"
									>
										{sending ? "Sende..." : `Senden → ${rollenKuerzel(selectedSchritt.empfaenger)}`}
									</Button>
								</Form>
							)}

							{lastResult && <SubmitResult result={lastResult} rolle={rolle} />}
						</>
					)}
				</CardContent>
			</Card>
		</div>
	);
}

function EmpfaengerAuswahl({
	aktiverEmpfaenger,
	kandidaten,
	schritt,
	onChange,
}: {
	aktiverEmpfaenger: Persona | null;
	kandidaten: Persona[];
	schritt: SchrittDef;
	onChange: (value: string) => void;
}) {
	if (!aktiverEmpfaenger) {
		return (
			<p className="text-destructive text-xs">
				Keine MP-ID für {rollenLabel(schritt.empfaenger)} gefunden.
			</p>
		);
	}

	if (kandidaten.length === 1) {
		return (
			<div className="rounded border border-border bg-muted/40 px-2 py-1.5 text-xs">
				<span className="text-muted-foreground">An: </span>
				<span className="font-medium">
					{aktiverEmpfaenger.vorname} {aktiverEmpfaenger.nachname}
				</span>
				<span className="text-muted-foreground"> · {aktiverEmpfaenger.firma} · </span>
				<span className="font-mono text-[11px]">{aktiverEmpfaenger.mpId}</span>
			</div>
		);
	}

	return (
		<div>
			<label className="mb-1 block text-xs text-muted-foreground" htmlFor="empfaenger-id">
				Empfänger-MP-ID
			</label>
			<select
				id="empfaenger-id"
				className="w-full rounded border border-input bg-background px-2 py-1.5 text-xs"
				value={aktiverEmpfaenger.mpId}
				onChange={(event) => onChange(event.target.value)}
			>
				{kandidaten.map((kandidat) => (
					<option key={kandidat.mpId} value={kandidat.mpId}>
						{kandidat.vorname} {kandidat.nachname} — {kandidat.firma}
					</option>
				))}
			</select>
			<p className="mt-1 font-mono text-[11px] text-muted-foreground">{aktiverEmpfaenger.mpId}</p>
		</div>
	);
}

function MaLoPicker({
	malos,
	selectedMalo,
	onChange,
}: {
	malos: MaLo[];
	selectedMalo: MaLo | undefined;
	onChange: (value: string) => void;
}) {
	return (
		<div className="space-y-1.5">
			<p className="text-xs text-muted-foreground">Marktlokation</p>
			<div className="grid gap-1.5 sm:grid-cols-2">
				{malos.map((malo) => (
					<button
						key={malo.id}
						type="button"
						aria-pressed={selectedMalo?.id === malo.id}
						className={`rounded border px-2 py-1.5 text-left text-xs transition-colors ${
							selectedMalo?.id === malo.id
								? "border-primary bg-primary/10"
								: "border-border hover:bg-accent"
						}`}
						onClick={() => onChange(malo.id)}
					>
						<span className="flex items-center justify-between gap-2">
							<span className="truncate font-medium">{malo.bezeichnung}</span>
							<Badge variant="secondary" className="h-5 px-1.5 text-[10px]">
								{malo.sparte}
							</Badge>
						</span>
						<span className="mt-0.5 block font-mono text-[11px] text-muted-foreground">
							{malo.id}
						</span>
					</button>
				))}
			</div>
			{selectedMalo && (
				<p className="line-clamp-2 text-muted-foreground text-xs leading-snug">
					{selectedMalo.story}
				</p>
			)}
		</div>
	);
}

function SchemaSkeleton() {
	return (
		<div className="space-y-3" aria-busy="true" aria-label="Schema lädt" role="status">
			<div className="h-4 w-24 animate-pulse rounded bg-muted" />
			<div className="h-8 animate-pulse rounded bg-muted" />
			<div className="h-4 w-32 animate-pulse rounded bg-muted" />
			<div className="h-8 animate-pulse rounded bg-muted" />
		</div>
	);
}

function SubmitResult({ result, rolle }: { result: ErstelleValidiertAntwort; rolle: string }) {
	const ahbProblems = collectAhbProblems(result);

	if (!result.ok) {
		return (
			<div
				role="alert"
				className="space-y-2 rounded border border-destructive/30 bg-destructive/10 p-3 text-destructive text-xs"
			>
				<p>{result.fehler ?? "Erstellen fehlgeschlagen."}</p>
				{ahbProblems.length > 0 && <AhbProblems problems={ahbProblems} />}
			</div>
		);
	}

	return (
		<div className="space-y-3 rounded border border-emerald-500/30 bg-emerald-500/10 p-3 text-xs">
			<p className="text-emerald-700 dark:text-emerald-300">
				Erstellt:{" "}
				{result.datei ? (
					<a
						className="font-mono underline-offset-2 hover:underline"
						href={outboxHref(rolle, result.datei)}
						rel="noreferrer"
						target="_blank"
					>
						{result.datei}
					</a>
				) : (
					<span>ohne Dateinamen</span>
				)}
			</p>
			<div>
				<p className="mb-1 text-muted-foreground">Wire-Output</p>
				<pre className="max-h-56 overflow-auto rounded bg-background p-3 font-mono text-[11px] leading-relaxed">
					{result.wire_format}
				</pre>
			</div>
			{ahbProblems.length > 0 && <AhbProblems problems={ahbProblems} />}
		</div>
	);
}

function AhbProblems({ problems }: { problems: AhbFeldErgebnis[] }) {
	return (
		<div>
			<p className="mb-1 font-medium">AHB-Probleme</p>
			<ul className="space-y-1">
				{problems.map((problem) => (
					<li
						key={`${problem.name}-${problem.segment_code ?? "segment"}-${problem.details ?? "ohne-detail"}-${problem.erwarteter_wert ?? "ohne-erwartung"}`}
						className="rounded border border-destructive/20 bg-background/60 px-2 py-1"
					>
						<span className="font-medium">{problem.name}</span>
						{problem.details ? <span>: {problem.details}</span> : null}
						{problem.erwarteter_wert ? (
							<span className="block text-muted-foreground">
								Erwartet: {problem.erwarteter_wert}
							</span>
						) : null}
					</li>
				))}
			</ul>
		</div>
	);
}

function IsoTextWidget(placeholder: string, pattern: string) {
	return function Widget(props: WidgetProps<FormFields>) {
		const value = typeof props.value === "string" ? props.value : "";

		return (
			<input
				aria-invalid={props.rawErrors && props.rawErrors.length > 0 ? "true" : undefined}
				className="w-full rounded border border-input bg-background px-2 py-1.5 text-sm"
				disabled={props.disabled}
				id={props.id}
				name={props.name}
				onBlur={(event) => props.onBlur(props.id, event.currentTarget.value)}
				onChange={(event) => props.onChange(event.currentTarget.value || undefined)}
				onFocus={(event) => props.onFocus(props.id, event.currentTarget.value)}
				pattern={pattern}
				placeholder={placeholder}
				readOnly={props.readonly}
				required={props.required}
				type="text"
				value={value}
			/>
		);
	};
}

function collectAhbProblems(result: ErstelleValidiertAntwort): AhbFeldErgebnis[] {
	return result.validierung?.ahb?.felder.filter((feld) => feld.urteil !== "Bestanden") ?? [];
}

function asFormFields(value: unknown): FormFields {
	return isRecord(value) ? value : {};
}

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === "object" && value !== null && !Array.isArray(value);
}

function outboxHref(rolle: string, datei: string): string {
	const base = import.meta.env.BASE_URL.replace(/\/$/, "");
	return `${base}/api/nachrichten/${encodeURIComponent(rolle)}/outbox/${encodeURIComponent(datei)}`;
}

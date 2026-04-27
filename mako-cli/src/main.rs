use clap::{Parser, Subcommand};

use mako_cli::{
	erstelle_nachricht, init, prozesse_json, schema_json, sende, status, verarbeite, verifiziere,
	verifiziere_batch,
};

#[derive(Parser)]
#[command(name = "mako", about = "MaKo-Simulator CLI")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Initialise a new Markt directory structure
	Init {
		/// Target directory (default: markt/)
		#[arg(default_value = "markt")]
		path: String,
	},
	/// Process an EDIFACT/JSON message through CONTRL, APERAK, and reducer
	Verarbeite {
		/// Path to the input file (EDIFACT or JSON)
		datei: String,
		/// Path to the markt directory
		#[arg(long, default_value = "markt")]
		markt: String,
	},
	/// Process all unverarbeitet messages in a role's inbox
	VerarbeiteAlle {
		/// Role directory name (e.g. netzbetreiber)
		rolle: String,
		/// Path to the markt directory
		#[arg(long, default_value = "markt")]
		markt: String,
	},
	/// Copy a file from one role's outbox to another role's inbox
	Sende {
		/// Sending role directory name
		von: String,
		/// Receiving role directory name
		an: String,
		/// File name to send (must exist in von/outbox/)
		datei: String,
		/// Path to the markt directory
		#[arg(long, default_value = "markt")]
		markt: String,
	},
	/// Show an overview of all roles and their message counts
	Status {
		/// Path to the markt directory
		#[arg(default_value = "markt")]
		markt: String,
	},
	/// Verify a single EDIFACT file against AHB reference data
	Verifiziere {
		/// Path to the EDIFACT file
		datei: String,
		/// Path to the referenzdaten directory
		#[arg(long, default_value = "referenzdaten")]
		referenzdaten: String,
	},
	/// Verify all .edi files in a directory tree
	VerifiziereBatch {
		/// Path to the directory containing .edi files
		verzeichnis: String,
		/// Path to the referenzdaten directory
		#[arg(long, default_value = "referenzdaten")]
		referenzdaten: String,
	},
	/// Print the engine's process catalog as JSON (for the test UI)
	ProzesseJson,
	/// Print the JSON schema for a payload type
	SchemaJson {
		/// Payload type name, e.g. UtilmdAnmeldung
		#[arg(long)]
		typ: String,
	},
	/// Build a wire message from JSON fields and write it to the sender outbox
	ErstelleNachricht {
		/// Active sender role slug, e.g. lieferant_neu
		#[arg(long)]
		rolle: String,
		/// Path to the markt directory
		#[arg(long)]
		markt: String,
		/// Payload type name, overriding stdin.typ when set
		#[arg(long)]
		typ: Option<String>,
		/// Copy to receiver inbox and create an acknowledgement immediately
		#[arg(long, default_value_t = true)]
		auto_zustellen: bool,
	},
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Init { path } => init::run(&path),
		Commands::Verarbeite { datei, markt } => {
			if let Err(e) = verarbeite::run(&datei, &markt) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
		Commands::VerarbeiteAlle { rolle, markt } => {
			if let Err(e) = verarbeite::run_alle(&markt, &rolle) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
		Commands::Sende { von, an, datei, markt } => {
			if let Err(e) = sende::run(&markt, &von, &an, &datei) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
		Commands::Status { markt } => {
			if let Err(e) = status::run(&markt) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
		Commands::Verifiziere { datei, referenzdaten } => {
			if let Err(e) = verifiziere::run(&datei, &referenzdaten) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
		Commands::VerifiziereBatch { verzeichnis, referenzdaten } => {
			if let Err(e) = verifiziere_batch::run(&verzeichnis, &referenzdaten) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
		Commands::ProzesseJson => prozesse_json::run(),
		Commands::SchemaJson { typ } => {
			if let Err(e) = schema_json::run(&typ) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
		Commands::ErstelleNachricht {
			rolle,
			markt,
			typ,
			auto_zustellen,
		} => {
			if let Err(e) = erstelle_nachricht::run_with_options(
				&rolle,
				&markt,
				auto_zustellen,
				typ.as_deref(),
			) {
				eprintln!("Fehler: {e}");
				std::process::exit(1);
			}
		}
	}
}

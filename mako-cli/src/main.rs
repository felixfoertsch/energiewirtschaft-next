use clap::{Parser, Subcommand};

mod event_mapping;
mod init;
mod sende;
mod state_store;
mod status;
mod verarbeite;

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
	}
}

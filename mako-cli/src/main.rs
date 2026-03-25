use clap::{Parser, Subcommand};

mod event_mapping;
mod init;
mod state_store;
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
	}
}

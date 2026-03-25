use clap::{Parser, Subcommand};

mod init;

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
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Init { path } => init::run(&path),
	}
}

use clap::{Parser, Subcommand};

mod check;
mod clean;
mod codegen;
mod fetch;
mod generate;

#[derive(Parser)]
#[command(name = "xtask", about = "Prismatica code generation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Fetch upstream colormap data
    Fetch {
        /// Collection name (default: all)
        collection: Option<String>,
    },
    /// Generate Rust source from fetched data
    Generate {
        /// Collection name (default: all)
        collection: Option<String>,
    },
    /// Run fetch + generate for all collections
    All,
    /// Remove fetched data and/or generated source files
    Clean {
        /// Remove only the data/.cache/ directory
        #[arg(long)]
        cache: bool,
        /// Remove only auto-generated .rs files from src/
        #[arg(long)]
        generated: bool,
    },
    /// Check that generated source files are up to date
    Check,
}

fn main() {
    let cli = Cli::parse();
    let root = codegen::project_root();

    match cli.command {
        Command::Fetch { collection } => {
            let name = collection.as_deref().unwrap_or("all");
            fetch::fetch_collection(&root, name);
        }
        Command::Generate { collection } => {
            let name = collection.as_deref().unwrap_or("all");
            generate::generate_collection(&root, name);
        }
        Command::All => {
            fetch::fetch_collection(&root, "all");
            generate::generate_collection(&root, "all");
        }
        Command::Clean { cache, generated } => {
            if let Err(e) = clean::run(&root, cache, generated) {
                eprintln!("Error: {e:#}");
                std::process::exit(1);
            }
        }
        Command::Check => {
            if let Err(e) = check::run(&root) {
                eprintln!("Error: {e:#}");
                std::process::exit(1);
            }
        }
    }
}

use clap::{Parser, Subcommand};

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
    }
}

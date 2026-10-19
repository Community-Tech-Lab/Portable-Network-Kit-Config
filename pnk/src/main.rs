use clap::{Parser, Subcommand};
mod status;
mod update;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Show the status of running Docker containers
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List running containers with their ID, name, status, and ports
    Status {
        /// Only show containers whose name contains this text
        #[arg(short, long)]
        container: Option<String>,
    },

    Install {
        /// Only show containers whose name contains this text
        #[arg(short, long)]
        container: Option<String>,
    },
    ///Updates the PNK CLI suite via the GitHub Repository
    Update {
        #[arg(short, long, default_value_t = 5000)]
        timeout: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Status { container }) => {
            status::status(container).await?;
        }

        Some(Commands::Install { container }) => match container {
            _ => {
                println!("Installing something...")
            }
        },

        Some(Commands::Update { timeout }) => update::update(*timeout).await,

        None => {}
    }

    Ok(())
}

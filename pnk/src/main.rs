use clap::{Parser, Subcommand};
use docker_wrapper::{DockerCommand, PsCommand};
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
}

async fn status(container_name: &Option<String>) -> Result<()> {
    let name = container_name.as_deref();
    match name {
        None => {
            let ps_cmd = PsCommand::new()
                .format_template("table {{.ID}}\\t{{.Names}}\\t{{.Status}}\\t{{.Ports}}")
                .execute()
                .await?;
            println!("{}", ps_cmd.stdout);
            Ok(())
        }

        Some(name) => {
            let ps_cmd = PsCommand::new()
                .filter(format!("name={}", name))
                .format_template("table {{.ID}}\\t{{.Names}}\\t{{.Status}}\\t{{.Ports}}")
                .execute()
                .await?;
            println!("{}", ps_cmd.stdout);
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Status { container }) => {
            status(container).await?;
        }

        None => {}
    }

    Ok(())
}

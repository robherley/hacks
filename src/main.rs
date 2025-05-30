use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "hacks")]
#[command(about = "CLI full of hacks that make my life easier")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// IP address utilities
    Ip {
        #[command(subcommand)]
        command: IpCommands,
    },
    /// UUID utilities
    Uuid {
        #[command(subcommand)]
        command: UuidCommands,
    },
    /// Decode msgpack from stdin
    #[command(name = "msgpack")]
    Msgpack {
        #[command(subcommand)]
        command: MsgpackCommands,
    },
    /// Convert CSV to markdown table
    #[command(name = "csv2md")]
    Csv2Md {
        /// Treat first row as header
        #[arg(long)]
        header: bool,
    },
}

#[derive(Subcommand)]
enum IpCommands {
    /// Get external IP address
    External,
    /// Get internal IP addresses
    Internal,
}

#[derive(Subcommand)]
enum UuidCommands {
    /// Generate a UUID
    Generate {
        /// UUID version to generate (1, 4, or 7)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=7))]
        version: Option<u8>,
    },
    /// Get information about a UUID
    Info {
        /// The UUID to analyze
        uuid: String,
    },
}

#[derive(Subcommand)]
enum MsgpackCommands {
    /// Decode base64 msgpack from stdin
    Decode,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ip { command } => match command {
            IpCommands::External => commands::ip::external().await,
            IpCommands::Internal => commands::ip::internal(),
        },
        Commands::Uuid { command } => match command {
            UuidCommands::Generate { version } => commands::uuid::generate(version),
            UuidCommands::Info { uuid } => commands::uuid::info(&uuid),
        },
        Commands::Msgpack { command } => match command {
            MsgpackCommands::Decode => commands::msgpack::decode(),
        },
        Commands::Csv2Md { header } => commands::csv2md::convert(header),
    }
}

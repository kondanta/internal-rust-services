// CLI Parser for the Spinner Service
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "arbiter",
    version,
    about = "The engine of the personal homepage"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Command>,

    /// Verbosity level for logging(-v -vv -vvv for more verbose output)
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(long, help = "Set the log level for the application")]
    pub log_level: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Provisions the whole cluster with the given configuration.
    #[command(verbatim_doc_comment)]
    Serve {
        /// The port to serve the application on.
        #[arg(short, long, default_value_t = 8080)]
        port: u16,

        /// The host to bind the server to.
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
    },
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

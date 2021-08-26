#[macro_use]
extern crate dotenv;

use std::error::Error;
use std::fmt;
use std::env;
use dotenv::dotenv;
use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use crate::netlify::process_cmd;

mod netlify;

#[derive(StructOpt, Debug)]
pub struct Cli {
    command: String,
    subcommand: String,
    target: Option<String>,
    #[structopt(short = "p", long)]
    platform: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CliConfig {
    netlify_token: Option<String>,
}

impl std::default::Default for CliConfig {
    fn default() -> Self { Self { netlify_token: Some("".into()) } }
}

#[derive(Debug, Clone)]
struct PlatformNotSpecifiedError;
impl Error for PlatformNotSpecifiedError {}

impl fmt::Display for PlatformNotSpecifiedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A platform must be provided. Valid options are: netlify")
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let cmd = Cli::from_args();
    dotenv::from_filename(".rubicon").unwrap();

    let config = CliConfig {
        netlify_token: env::var("NETLIFY_TOKEN").ok()
    };

    match cmd.platform.as_deref().unwrap_or("") {
        "netlify" => process_cmd(cmd, config).await,
        _ => Result::Err(PlatformNotSpecifiedError.into()),
    }
}

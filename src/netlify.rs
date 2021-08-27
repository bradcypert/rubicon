use crate::{Cli, CliConfig};

use std::error::Error;
use std::fmt;
use colored::*;
use netlify_rust::apis::configuration::Configuration;
use netlify_rust::apis as netlify;

#[derive(Debug, Clone)]
struct CommandNotFoundError;
impl Error for CommandNotFoundError {}

impl fmt::Display for CommandNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A command must be provided. Valid options are: list")
    }
}

#[derive(Debug, Clone)]
struct SubcommandNotFoundError;
impl Error for SubcommandNotFoundError {}

impl fmt::Display for SubcommandNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A subcommand must be provided. Valid options are: sites")
    }
}

pub async fn process_cmd(cmd: Cli, cli_config: CliConfig) -> Result<(), Box<dyn Error>> {
    let mut config = Configuration::new();
    config.oauth_access_token = cli_config.netlify_token;
    match &*cmd.command {
        "list" => handle_list(cmd, config).await,
        // "status" => handle_status(cmd, config),
        _ => Result::Err(CommandNotFoundError.into()),
    }
}

async fn handle_list(cmd: Cli, config: Configuration) -> Result<(), Box<dyn Error>> {
    match &*cmd.subcommand {
        "sites" => list_resources(cmd, config).await,
        "deploys" => list_deploys(cmd, config).await,
        _ => Result::Err(SubcommandNotFoundError.into()),
    }
}

async fn handle_status(cmd: Cli, config: Configuration) -> Result<(), Box<dyn Error>> {
    Result::Ok(())
}

// List sites for netlify user
async fn list_resources(cmd: Cli, config: Configuration) -> Result<(), Box<dyn Error>> {
    let sites_req = netlify::site_api::list_sites(&config, cmd.target.as_deref(), Some("all"), None, None).await;

    match sites_req {
        Ok(sites) => {
            for site in sites.iter() {
                println!("{}", site.name.as_ref().unwrap());
            }
            Result::Ok(())
        }
        Err(error) => {
            println!("Something went wrong while fetching sites!\n {}", error);
            Result::Err(error.into())
        }
    }
}

// Get status and data about current site
async fn site_status(cmd: Cli) -> Result<(), Box<dyn Error>> {
    Result::Ok(())
}

async fn list_deploys(cmd: Cli, config: Configuration) -> Result<(), Box<dyn Error>> {
    let sites_req = netlify::site_api::list_sites(&config, cmd.target.as_deref(), Some("all"), None, None).await;

    match sites_req {
        Ok(sites) => {
            let site = sites.iter().find(|&site| site.name == cmd.target).unwrap();
            let site_id = site.id.as_ref().unwrap();
            let deploy_details = netlify::deploy_api::list_site_deploys(&config, site_id, None, None).await?;
            
            let last_five: Vec<&netlify_rust::models::Deploy> = deploy_details.iter().take(5).collect();

            println!("Deploys for {}:", site.name.as_ref().unwrap());
            for deploy in deploy_details.iter().take(5) {
                // TODO: We may not have branch info if its a manual upload
                println!("[Build ID: {}] branch {} - {}", deploy.build_id.as_ref().unwrap_or(&String::from("unknown")).blue(), deploy.branch.as_ref().unwrap_or(&String::from("unknown")).yellow(), deploy.state.as_ref().unwrap());
                println!("|- Commit Ref: {}", deploy.commit_ref.as_ref().unwrap_or(&String::from("unknown")).cyan());
                println!("|- Commit URL: {}", deploy.commit_url.as_ref().unwrap_or(&String::from("unknown")).cyan());
                println!("|- Created: {}", deploy.created_at.as_ref().unwrap().magenta());
                println!("|- Updated: {}", deploy.created_at.as_ref().unwrap().magenta());
                println!("|- Published: {}", deploy.created_at.as_ref().unwrap().magenta());
                println!("");
            }

            Result::Ok(())
        }
        Err(error) => {
            println!("Something went wrong while fetching sites!\n {}", error);
            Result::Err(error.into())
        }
    }
}

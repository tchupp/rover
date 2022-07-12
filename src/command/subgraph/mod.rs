mod check;
mod delete;
mod fetch;
mod init;
mod introspect;
mod list;
mod publish;

use clap::Parser;
use serde::Serialize;

use crate::command::RoverOutput;
use crate::utils::client::StudioClientConfig;
use crate::Result;

use rover_client::shared::GitContext;

#[derive(Debug, Serialize, Parser)]
pub struct Subgraph {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, Parser)]
pub enum Command {
    /// Check for build errors and breaking changes caused by an updated subgraph schema
    /// against the federated graph in the Apollo graph registry
    Check(check::Check),

    /// Delete a subgraph from the Apollo registry and trigger composition in the graph router
    Delete(delete::Delete),

    /// Fetch a subgraph schema from the Apollo graph registry
    Fetch(fetch::Fetch),

    /// Initialize a .apollo directory for a subgraph in the current directory
    Init(init::Init),

    /// Introspect a running subgraph endpoint to retrieve its schema definition (SDL)
    Introspect(introspect::Introspect),

    /// List all subgraphs for a federated graph
    List(list::List),

    /// Publish an updated subgraph schema to the Apollo graph registry and trigger composition in the graph router
    Publish(publish::Publish),
}

impl Subgraph {
    pub fn run(
        &self,
        client_config: StudioClientConfig,
        git_context: GitContext,
    ) -> Result<RoverOutput> {
        match &self.command {
            Command::Check(command) => command.run(client_config, git_context),
            Command::Delete(command) => command.run(client_config),
            Command::Fetch(command) => command.run(client_config),
            Command::Init(command) => command.run(),
            Command::Introspect(command) => command.run(client_config.get_reqwest_client()),
            Command::List(command) => command.run(client_config),
            Command::Publish(command) => command.run(client_config, git_context),
        }
    }
}

use clap::{ArgMatches, Command};
use rango_sdk::client::Client;

mod blockchains;
mod protocols;
mod swappers;

pub fn cmd() -> Command {
    Command::new("show")
        .subcommand_required(true)
        .subcommand(blockchains::cmd())
        .subcommand(swappers::cmd())
        .subcommand(protocols::cmd())
}

pub async fn process(matches: &ArgMatches, client: &Client) {
    match matches.subcommand() {
        Some(("blockchains", sub_matches)) => blockchains::process(sub_matches, client).await,
        Some(("swappers", sub_matches)) => swappers::process(sub_matches, client).await,
        Some(("protocols", sub_matches)) => protocols::process(sub_matches, client).await,
        _ => unreachable!(),
    }
}

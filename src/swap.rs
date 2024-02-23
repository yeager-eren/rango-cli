use clap::{ArgMatches, Command};
use rango_sdk::client::Client;

pub fn cmd() -> Command {
    Command::new("swap").about("Do your swap")
}

pub async fn process(_matches: &ArgMatches, _client: &Client) {
    println!("Not implemented yet.")
}

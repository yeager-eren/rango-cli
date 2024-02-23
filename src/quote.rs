use clap::{ArgMatches, Command};
use rango_sdk::client::Client;

pub fn cmd() -> Command {
    Command::new("quote").about("Get a quote for your desired swap.")
}

pub async fn process(_matches: &ArgMatches, _client: &Client) {
    println!("Not implemented yet.")
}

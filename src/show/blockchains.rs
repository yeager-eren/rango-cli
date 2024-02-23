use clap::{ArgMatches, Command};
use console::style;
use rango_sdk::client::Client;

pub fn cmd() -> Command {
    Command::new("blockchains").about("Supported blockchains by Rango")
}

pub async fn process(_matches: &ArgMatches, client: &Client) {
    let blockchains = client.chains().await.map_or_else(
        |_e| format!("{}", style("Request failed.").red()),
        |blockchains| format!("{}", style(blockchains.len()).green()),
    );

    // TODO: show blockchain names. Needs some changes on rango-sdk
    println!(
        "{}",
        style("THIS WILL SHOW BLOCKCHAIN NAMES EVENTUALLY.").yellow()
    );
    println!("Blockchains: \n {}", blockchains);
}

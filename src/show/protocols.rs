use clap::{ArgMatches, Command};
use console::style;
use rango_sdk::client::Client;

pub fn cmd() -> Command {
    Command::new("protocols").about("Supported messaging protocols by Rango")
}

pub async fn process(_matches: &ArgMatches, client: &Client) {
    let messaging_protocols = client.messaging_protocols().await.map_or_else(
        |_e| format!("{}", style("Request failed.").red()),
        |messaging_protocols| {
            let names: Vec<String> = messaging_protocols
                .protocols
                .iter()
                .map(|protocol| format!("- {}", protocol.id.clone()))
                .collect();
            format!("{}", names.join("\n"))
        },
    );

    println!("Supported messaging protocols:");
    println!("{messaging_protocols}");
}

use clap::{ArgMatches, Command};
use console::style;
use rango_sdk::client::Client;

pub fn cmd() -> Command {
    Command::new("info").about("Showing some general infromation about rango")
}

pub async fn process(_matches: &ArgMatches, client: &Client) {
    let blockchains = client.chains().await.map_or_else(
        |_e| format!("{}", style("Request failed.").red()),
        |v| format!("{}", style(v.len()).green()),
    );
    let swappers = client.swappers().await.map_or_else(
        |_e| format!("{}", style("Request failed.").red()),
        |v| format!("{}", style(v.len()).green()),
    );
    let messaging_protocols = client.messaging_protocols().await.map_or_else(
        |_e| format!("{}", style("Request failed.").red()),
        |v| {
            let names: Vec<String> = v
                .protocols
                .iter()
                .map(|protocol| protocol.id.clone())
                .collect();
            format!("{}", style(names.join(",")).green())
        },
    );

    println!("Supported blockchains: {} ", blockchains);
    println!("Supported swappers: {}", swappers);
    println!("Supported protocols: {}", messaging_protocols);
}

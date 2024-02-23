use clap::{ArgMatches, Command};
use console::style;
use rango_sdk::client::Client;

pub fn cmd() -> Command {
    Command::new("swappers").about("Supported swappers by Rango")
}

pub async fn process(_matches: &ArgMatches, client: &Client) {
    let swappers = client.swappers().await.map_or_else(
        |_e| format!("{}", style("Request failed.").red()),
        |swappers| {
            let names: Vec<String> = swappers
                .iter()
                .map(|swapper| format!("- {}", swapper.title.clone()))
                .collect();
            format!("{}", names.join("\n"))
        },
    );
    println!("Supported swappers:");
    println!("{swappers}");
}

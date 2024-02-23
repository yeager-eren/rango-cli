mod balance;
mod connect;
mod info;
mod quote;
mod show;
mod swap;

#[tokio::main]
async fn main() {
    let rango = rango_sdk::client::Client::new(
        "uuid-uuid-uuid",
        "4a624ab5-16ff-4f96-90b7-ab00ddfc342c",
        None,
    );

    let cmd = clap::Command::new("rango")
        .about("A client to use Rango Exchange from terminal")
        .subcommand_required(true)
        .subcommand(info::cmd())
        .subcommand(show::cmd())
        .subcommand(connect::cmd())
        .subcommand(balance::cmd())
        .subcommand(quote::cmd())
        .subcommand(swap::cmd());

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("info", sub_matches)) => info::process(sub_matches, &rango).await,
        Some(("show", sub_matches)) => show::process(sub_matches, &rango).await,
        Some(("connect", sub_matches)) => connect::process(sub_matches, &rango).await,
        Some(("balance", sub_matches)) => balance::process(sub_matches, &rango).await,
        Some(("quote", sub_matches)) => quote::process(sub_matches, &rango).await,
        Some(("swap", sub_matches)) => swap::process(sub_matches, &rango).await,
        _ => unreachable!(),
    }
}

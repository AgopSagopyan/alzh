mod cli;
mod commands;
mod tui;

mod detect;
mod manifest;
mod scan;

use clap::Parser;
use cli::{Cli, Command};
use std::process;

// programin ana fonksiyonu
// Hata yakalama sistemi eklendi
fn main() {
    if let Err(why) = run() {
        eprintln!("hata {why}");
        process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let cli = Cli::parse();

    // argumanlar burdan geliyor "board" "init" gibi
    match cli.command {
        Command::Board { path, cli } => commands::board::run(path.as_deref(), cli),
        Command::Init => commands::init::run(),
        Command::Show { name } => commands::show::run(&name),
        Command::Import => commands::import::run(),
        Command::NOTE { name } => commands::note::run(&name),
    }?;

    Ok(())
}

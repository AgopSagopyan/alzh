mod cli;
mod commands;
mod project;
mod tui;

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
        Command::Board => tui::app::run(),
        Command::Init => commands::init::run(),
        Command::Show { name } => commands::show::run(&name),
        Command::Import => commands::import::run(),
    }?;

    Ok(())
}

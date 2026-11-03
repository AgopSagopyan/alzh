use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Proje panosunu acar
    Board,
    /// Bulundugun klasorde alzh.toml olusturur
    Init,
    /// alzh.toml icerigini gosterir
    Show {
        name: String,
    },
    Import,
}

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Proje panosunu acar
    Board {
        /// Acilacak proje klasoru, verilmezse tum projeler listelenir
        path: Option<PathBuf>,
        /// TUI yerine duz liste yazdirir
        #[arg(long)]
        cli: bool,
    },
    NOTE {
        name: String,
    },
    /// Bulundugun klasorde alzh.toml olusturur
    Init,
    /// alzh.toml icerigini gosterir
    Show {
        name: String,
    },
    Import,
}

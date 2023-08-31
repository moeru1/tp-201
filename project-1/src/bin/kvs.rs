use clap::{Parser, Subcommand};
use kvs::KvStore;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands { 
    /// Sets the value of a string key to a string
    Set {
        key: String,
        value: String,
    },
    /// Get the string value of a given string key
    Get {
        key: String,
    },
    /// Remove a given key 
    Rm {
        key: String,
    }

}

fn main() {
    let cli = Cli::parse();
    run(cli).unwrap();
}

fn run(cli: Cli) -> Option<String> {
    let mut kv_store = KvStore::new();
    eprintln!("unimplemented");
    exit(1);

    match cli.command {
        Commands::Set{key, value} => {
            kv_store.set(key, value)
        },
        Commands::Get{key} => {
            kv_store.get(key)
        }
        Commands::Rm{key} => {
            kv_store.remove(key)
        }
    }
}

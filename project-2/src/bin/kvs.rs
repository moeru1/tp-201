use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands { 
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

fn main() -> Result<()>{
    let cli = Cli::parse();
    run(cli)
}

fn run(cli: Cli) -> Result<()> {
    let mut kv_store = KvStore::open("aa")?;

    match cli.command {
        CliCommands::Set{key, value} => {
            kv_store.set(key, value)?;
            Ok(())
        },
        CliCommands::Get{key} => {
            let val = kv_store.get(key)?;
            match val {
                Some(a) => println!("{}", a),
                None => println!("Key not found"),
            }
            Ok(())
        }
        CliCommands::Rm{key} => {
            unimplemented!()
        }
    }
}

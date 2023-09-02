use clap::{Parser, Subcommand};
use kvs::{KvStore, Result, KeyNotFound};
use std::path::PathBuf;
use std::process::{ExitCode, Termination};
use anyhow::anyhow;

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

//struct AppResult(Result<()>);
//
//impl Termination for AppResult {
//    fn report(self) -> ExitCode {
//        match self {
//            Ok(_) => ExitCode::SUCCESS,
//            Err(e) => ExitCode::FAILURE
//        }
//    }
//}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let result = run(cli);
    if result
        .as_ref()
            .is_err_and(|e| e.downcast_ref::<KeyNotFound>().is_some()) {
        std::process::exit(1)
    }
    result
}

fn run(cli: Cli) -> Result<()> {
    let mut kv_store = KvStore::open(PathBuf::from("."))?;

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
            let a = kv_store.remove(key);
            let Err(e) = a else {
                return Ok(());
            };

            match e.downcast_ref::<KeyNotFound>() {
                Some(_) => {
                    println!("Key not found");
                    Err(e)
                }
                None => Err(e)
            }

        }
    }
}

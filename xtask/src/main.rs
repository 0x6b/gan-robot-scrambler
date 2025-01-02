use std::{
    collections::HashMap,
    env::var,
    fs::{read_to_string, remove_dir_all},
    path::PathBuf,
    process::Command,
};

use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;
use serde::Deserialize;

#[derive(Debug, Parser)]
#[clap(about, override_usage = "cargo <x|xtask> [OPTIONS] <COMMAND>")]
struct Args {
    #[clap(subcommand)]
    sub_command: SubCommand,

    #[clap(long, short, default_value = "xtask-config.json")]
    config: PathBuf,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Build and flash the program to the board
    Run {
        /// Build artifacts in release mode, with optimizations
        #[clap(long)]
        release: bool,
    },

    /// Build the program
    Build {
        /// Build artifacts in release mode, with optimizations
        #[clap(long)]
        release: bool,
    },

    /// Remove the target directory
    Clean {
        /// Remove `.embuild` directory along with the target directory
        #[clap(long)]
        all: bool,
    },

    /// Open a serial console
    SerialConsole {
        #[clap(long, default_value = "espflash")]
        espflash_path: String,
    },
}

#[derive(Debug, Deserialize)]
struct Config {
    pub env: Option<HashMap<String, String>>,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let Args { sub_command, config } = Args::parse();
    let config: Config = serde_json::from_str(&read_to_string(config)?)?;
    info!("{config:#?}");

    let mut command = match sub_command {
        SubCommand::SerialConsole { ref espflash_path } => Command::new(espflash_path),
        _ => Command::new(var("CARGO").unwrap_or_else(|_| "cargo".to_string())),
    };

    if let Some(env) = config.env {
        env.iter().for_each(|(key, value)| {
            command.env(key, value);
        });
    }

    match sub_command {
        SubCommand::Run { release } => {
            command.arg("run");
            if release {
                command.arg("--release");
            }
        }
        SubCommand::Build { release } => {
            command.arg("build");
            if release {
                command.arg("--release");
            }
        }
        SubCommand::SerialConsole { .. } => {
            command.arg("monitor");
        }
        SubCommand::Clean { all } => {
            if all {
                if let Ok(workspace_root) = var("CARGO_WORKSPACE_DIR") {
                    remove_dir_all(PathBuf::from(workspace_root).join(".embuild"))?;
                }
            }
            command.arg("clean");
        }
    }

    command.status()?;
    Ok(())
}

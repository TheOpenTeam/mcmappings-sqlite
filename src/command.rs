/*
 *
 *  * Created: 2026-2-22 3:37:17
 *  * File: command.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mcmappings-sqlite")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}
#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Create a new mappings database")]
    Create {
        #[arg(default_value = "mappings.db")]
        path: String
    },
    #[command(about = "Append vanilla mappings to a database")]
    Append {
        #[arg(short, long)]
        inputs : Vec<String>,
        #[arg(short, long, default_value = "mappings.db")]
        db: String,
        #[arg(short, long)]
        version: String,
        #[arg(short, long, default_value_t = false)]
        debug: bool,
    }
}
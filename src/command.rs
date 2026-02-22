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
    Create {
        #[arg(default_value = "mappings.db")]
        path: String
    },
}
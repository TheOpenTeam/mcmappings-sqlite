/*
 *
 *  * Created: 2026-2-22 3:29:8
 *  * File: main.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
pub mod command;
pub mod db;
pub mod mapping;
pub mod resolvers;

use clap::Parser;
use log::{info, warn, LevelFilter};
use crate::command::{Cli, Commands};
use crate::db::{create_empty_db};
use crate::mapping::{append_mappings};
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Create {path} => {env_logger::builder().filter_level(LevelFilter::Info).init(); create_empty_db(&path)?},
        Commands::Append {inputs, db, version, debug } => {
            if debug { env_logger::builder().filter_level(LevelFilter::Debug).init(); } else {env_logger::builder().filter_level(LevelFilter::Info).init(); }
            append_mappings(inputs, &db, &version)?
        },
    }
    Ok(())
}
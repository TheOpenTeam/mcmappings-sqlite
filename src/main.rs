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

use clap::Parser;
use log::{info, warn};
use crate::command::{Cli, Commands};
use crate::db::{create_empty_db};
fn main() -> anyhow::Result<()> {
    unsafe { std::env::set_var("RUST_LOG", "info"); } // 强制所有级别
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Create {
            path
        } => {
            create_empty_db(&path)?;
        }
    }
    Ok(())
}
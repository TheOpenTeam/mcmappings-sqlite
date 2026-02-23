/*
 *
 *  * Created: 2026-2-22 4:41:21
 *  * File: mapping.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use clap::ValueEnum;
use std::{fs, fs::File, path};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use log::info;
use proguard::{ProguardMapper, ProguardMapping, ProguardRecord, ProguardRecordIter};
use rusqlite::{Connection, params};
use crate::db::create_empty_db;


#[derive(Clone, ValueEnum)]
pub enum MappingType {
    Vanilla,
    Fabric,
    Forge,
}

pub(crate) fn append_mappings(inputs: Vec<String>, path: &str, version: &str) -> anyhow::Result<()> {
    if !Path::new(path).exists() {
        create_empty_db(path)?;
    }

    let conn = Connection::open(path)?;
    
}


fn detect_platform(path: &str) -> anyhow::Result<MappingType> {
    // 通过文件后缀来判断
    match path {
        path if path.ends_with(".srg") || path.ends_with(".tsrg") => Ok(MappingType::Forge),
        path if path.ends_with(".txt") || path.ends_with(".mappings") => Ok(MappingType::Vanilla),
        path if path.ends_with(".tiny") => Ok(MappingType::Fabric),
        _ => Err(anyhow::Error::msg(format!("Unknown mapping type while detecting {path:?}"))),
    }
}

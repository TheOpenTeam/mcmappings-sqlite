/*
 *
 *  * Created: 2026-2-24 10:21:29
 *  * File: tiny.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::fs;
use log::{error, info};
use rusqlite::Connection;
pub fn process_tiny(input: &str, db: &str, version: &str) -> anyhow::Result<(usize)> {
    // 判断tiny版本
    let content = fs::read_to_string(input)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut line_len: usize = 0;
    if let Some(first_line) = lines.first() {
        if first_line.starts_with("v1") {
            // v1
            line_len += process_tiny_v1(input, db, version)?;
        } else if first_line.starts_with("tiny") {
            // TODO: tiny v2
        } else {
            return Err(anyhow::Error::msg(format!("Unknown tiny version in {}", input)));
        }
    } else {
        return Err(anyhow::Error::msg(format!("Empty file: {}", input)));
        }


    Ok(line_len)
}
fn process_tiny_v1(input: &str, db: &str, version: &str) -> anyhow::Result<(usize)> {
    let conn = Connection::open(db)?;
    conn.execute("PRAGMA synchronous = OFF", [])?;
    conn.execute("PRAGMA reverse_unordered_selects = 1", [])?;
    conn.execute("PRAGMA temp_store = MEMORY", [])?;
    conn.execute("BEGIN TRANSACTION", [])?;
    let mut class_pre = conn.prepare("INSERT OR REPLACE INTO fabric_classes_v1 (version, official, named, intermediary) VALUES (?1, ?2, ?3, ?4)")?;
    let mut method_pre = conn.prepare("INSERT OR REPLACE INTO fabric_methods_v1 (class_id, parent_class, desc, official, named, intermediary) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
    let mut field_pre = conn.prepare("INSERT OR REPLACE INTO fabric_fields_v1 (class_id, parent_class, field_type, official, named, intermediary) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
    let mut named_index = 0;
    let mut intermediary_index = 0;
    let mut state: Option<i64> = None; // 存储类ID
    let file = fs::read_to_string(input)?;
    let lines = file.lines().collect::<Vec<&str>>();
    let line_len = lines.len();
    for line in lines {
        let split: Vec<&str> = line.split('\t').collect();
        match split[0] {
            // 类
            "v1" => {
                intermediary_index = split.iter().position(|&x| x == "intermediary")
                    .ok_or_else(|| anyhow::anyhow!("Missing intermediary in Tiny v1 header"))?;
                named_index = split.iter().position(|&x| x == "named")
                    .ok_or_else(|| anyhow::anyhow!("Missing named in Tiny v1 header"))?;
            },
            "CLASS" => {
                let official = split[1];
                let named = split[named_index];
                let intermediary = split[intermediary_index];
                class_pre.execute((version, official, named, intermediary))?;
                let id = conn.last_insert_rowid();
                state = Some(id);
                info!("Processed class(Tiny v1, ID:{}) {} -> {} -> {}", id, official, named, intermediary);
            }
            "METHOD" => {
                if let Some(id) = state {
                    let parent_class = split[1];
                    let desc = split[2];
                    let official = split[3];
                    let intermediary = split[intermediary_index + 2];
                    let named = split[named_index + 2];
                    method_pre.execute((id, parent_class, desc, official, named, intermediary))?;
                    info!("Processed method(Tiny v1, ID:{}) {} -> {} -> {}", conn.last_insert_rowid(), official, named, intermediary);
                }
            }
            "FIELD" => {
                if let Some(id) = state {
                    let parent_class = split[1];
                    let field_type = split[2];
                    let official = split[3];
                    let intermediary = split[intermediary_index + 2];
                    let named = split[named_index + 2];
                    field_pre.execute((id, parent_class, field_type, official, named, intermediary))?;
                    info!("Processed field(Tiny v1, ID:{}) {} -> {} -> {}", conn.last_insert_rowid(), official, named, intermediary);
                }
            }
            &_ => error!("Unknown line type {}", line)


        }
    }
    Ok(line_len)
}
/*
 *
 *  * Created: 2026-2-24 10:21:29
 *  * File: tiny.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::collections::HashMap;
use std::fs;
use log::{debug, error, info};
use rusqlite::Connection;
#[derive(Debug, PartialEq)]
enum State {
    Class,
    Method,
    None,
}
pub fn process_tiny(input: &str, db: &str, version: &str) -> anyhow::Result<(usize)> {
    // 判断tiny版本
    let content = fs::read_to_string(input)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut line_len: usize = 0;
    if let Some(first_line) = lines.first() {
        if first_line.starts_with("v1") {
            // v1
            info!("Detected tiny v1 in {}", input);
            line_len += process_tiny_v1(lines, db, version)?;
        } else if first_line.starts_with("tiny") {
            // v2
            info!("Detected tiny v2 in {}", input);
            line_len += process_tiny_v2(lines, db, version)?;

        } else {
            return Err(anyhow::Error::msg(format!("Unknown tiny version in {}", input)));
        }
    } else {
        return Err(anyhow::Error::msg(format!("Empty file: {}", input)));
        }


    Ok(line_len)
}
fn process_tiny_v1(lines: Vec<&str>, db: &str, version: &str) -> anyhow::Result<(usize)> {
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
    let line_len = lines.len();
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        match split[0] {
            // 类
            "v1" => {
                intermediary_index = split.iter().position(|&x| x == "intermediary")
                    .ok_or_else(|| anyhow::anyhow!("Missing intermediary in Tiny v1 header"))?;
                named_index = split.iter().position(|&x| x == "named")
                    .ok_or_else(|| anyhow::anyhow!("Missing named in Tiny v1 header"))?;
                debug!("Processed header(Tiny v1)");
            },
            "CLASS" => {
                let official = split[1];
                let named = split[named_index];
                let intermediary = split[intermediary_index];
                class_pre.execute((version, official, named, intermediary))?;
                let id = conn.last_insert_rowid();
                state = Some(id);
                debug!("Processed class(Tiny v1, ID:{}) {} -> {} -> {}", id, official, named, intermediary);
            }
            "METHOD" => {
                if let Some(id) = state {
                    let parent_class = split[1];
                    let desc = split[2];
                    let official = split[3];
                    let intermediary = split[intermediary_index + 2];
                    let named = split[named_index + 2];
                    method_pre.execute((id, parent_class, desc, official, named, intermediary))?;
                    debug!("Processed method(Tiny v1, ID:{}) {} -> {} -> {}", conn.last_insert_rowid(), official, named, intermediary);
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
                    debug!("Processed field(Tiny v1, ID:{}) {} -> {} -> {}", conn.last_insert_rowid(), official, named, intermediary);
                }
            }
            &_ => error!("Unknown line type {}", line)


        }
    }
    conn.execute("COMMIT", [])?;
    Ok(line_len)
}
fn process_tiny_v2(lines: Vec<&str>, db: &str, version: &str) -> anyhow::Result<(usize)> {
    let conn = Connection::open(db)?;
    conn.execute("PRAGMA synchronous = OFF", [])?;
    conn.execute("PRAGMA reverse_unordered_selects = 1", [])?;
    conn.execute("PRAGMA temp_store = MEMORY", [])?;
    conn.execute("BEGIN TRANSACTION", [])?;
    let mut class_pre = conn.prepare("INSERT OR REPLACE INTO fabric_classes_v2 (version, intermediary, named) VALUES (?1, ?2, ?3)")?;
    let mut field_pre = conn.prepare("INSERT OR REPLACE INTO fabric_fields_v2 (class_id, field_type, intermediary, named) VALUES (?1, ?2, ?3, ?4)")?;
    let mut method_pre = conn.prepare("INSERT OR REPLACE INTO fabric_methods_v2 (class_id, desc, intermediary, named) VALUES (?1, ?2, ?3, ?4)")?;
    let mut parameter_pre = conn.prepare(
        "UPDATE fabric_methods_v2
        SET parameters = ?2
        WHERE method_id = ?1")?;
    let line_len = lines.len();
    let mut state = State::None;
    let mut intermediary_index = 0;
    let mut named_index = 0;
    let mut current_class_id = 0;
    let mut parameters: HashMap<i64, String> = HashMap::new();
    let mut current_method_id = 0;
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        match split[0] {
            "tiny" => {
                intermediary_index = split.iter().position(|&x| x == "intermediary").ok_or(anyhow::anyhow!("Missing intermediary in Tiny v2 header"))?;
                named_index = split.iter().position(|&x| x == "named").ok_or(anyhow::anyhow!("Missing named in Tiny v2 header"))?;
                debug!("Processed header(Tiny v2)");
            }
            // 类或注释
            "c" => {
                // -2
                if state == State::Method {
                    continue; // 如果是注释就跳过了
                }
                state = State::Class;
                let intermediary = split[intermediary_index - 2];
                let named = split[named_index - 2];
                class_pre.execute((version, intermediary, named))?;
                current_class_id = conn.last_insert_rowid();
                debug!("Processed class(Tiny v2, ID:{}) -> {} -> {}", current_class_id, intermediary, named);
            }
            // 字段
            "f" => {
                let field_type = split[1];
                let intermediary = split[intermediary_index - 1];
                let named = split[named_index - 1];
                field_pre.execute((current_class_id, field_type, intermediary, named))?;
                debug!("Processed field(Tiny v2, ID:{}) -> {} -> {}", conn.last_insert_rowid(), intermediary, named);
            }
            // 方法
            "m" => {
                if current_method_id != 0 && !parameters.is_empty() {
                    let params_json = serde_json::to_string(&parameters)?;
                    parameter_pre.execute((current_method_id, params_json))?;
                }
                state = State::Method;
                let desc = split[1];
                let intermediary = split[intermediary_index - 1];
                let named = split[named_index - 1];
                method_pre.execute((current_class_id, desc, intermediary, named))?;
                current_method_id = conn.last_insert_rowid();
                debug!("Processed method(Tiny v2, ID:{}) -> {} -> {}", current_method_id, intermediary, named);
            }
            // 方法参数
            "p" => {
                let idx = split[1].parse::<i64>()?;
                let name = split[2];
                parameters.insert(idx, name.to_string());
                debug!("Processed parameter(Tiny v2, ID:{}) -> {} -> {}", current_method_id, idx, name);
            }
            &_ => error!("Unknown line type {}", line)
        }
    }
    if current_method_id != 0 && !parameters.is_empty() {
        let params_json = serde_json::to_string(&parameters)?;
        parameter_pre.execute((current_method_id, params_json))?; // 最后一个方法参数
    }
    conn.execute("COMMIT", [])?;
    Ok(line_len)
}

/*
 *
 *  * Created: 2026-2-23 9:43:23
 *  * File: proguard.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::cmp::PartialEq;
use std::fs;
use log::{error, info};
use rusqlite::Connection;
use serde_json::Value;

#[derive(Debug, PartialEq)]
enum State {
    Metadata,
    Class,
    None,
}

pub fn process_proguard(path: &str, db: &str, version: &str) -> anyhow::Result<(usize)> {

    let conn = Connection::open(db)?;
    // 优化
    conn.execute("PRAGMA synchronous = OFF", [])?;
    conn.execute("PRAGMA reverse_unordered_selects = 1", [])?;
    conn.execute("PRAGMA temp_store = MEMORY", [])?;
    conn.execute("BEGIN TRANSACTION", [])?;
    // 预编译
    let mut class_pre = conn.prepare("INSERT OR REPLACE INTO vanilla_classes (version, original, obfuscated) VALUES (?1, ?2, ?3)")?;
    let mut method_pre = conn.prepare("INSERT OR REPLACE INTO vanilla_methods (class_id, original, obfuscated, return_type, parameter_types) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    let mut method_meta_pre = conn.prepare("INSERT OR REPLACE INTO vanilla_methods (source_file, original, obfuscated, return_type, parameter_types) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    let mut field_pre = conn.prepare("INSERT OR REPLACE INTO vanilla_fields (class_id, original, obfuscated, field_type) VALUES (?1, ?2, ?3, ?4)")?;

    let file = fs::read_to_string(path)?;
    let mut lines: Vec<&str> = file.lines().skip(1).collect();
    let line_len = lines.len();
    let mut current_class_obfuscated: String = String::new();
    let mut current_class_original: String = String::new();
    let mut current_class_id = 0;
    let mut current_metadata_source_file: String = String::new();
    let mut state = State::None;
    for line in lines {
        match line {
            // 不以空格开头则为类
            line if !line.starts_with(" ") && !line.starts_with("#") => {
                state = State::Class;
                current_metadata_source_file = String::from("");
                let class = line.trim().split(" -> ").collect::<Vec<&str>>();
                current_class_obfuscated = class[1].replace(":", "");
                current_class_original = class[0].to_string();
                //conn.execute("INSERT OR REPLACE INTO vanilla_classes (version, original, obfuscated) VALUES (?1, ?2, ?3)", (version, &current_class_original, &current_class_obfuscated))?;
                class_pre.execute((version, &current_class_original, &current_class_obfuscated))?;
                current_class_id = conn.last_insert_rowid();
                info!("Processed class(ID: {}), : {} ->  {}", current_class_id, current_class_original, current_class_obfuscated);
            }
            // 元数据，标记状态
            line if line.starts_with("#") => {
                state = State::Metadata;
                let v: Value = serde_json::from_str(&line.trim()[1..])?;
                current_metadata_source_file = v["fileName"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
            },

            // 字段与方法
            line if line.starts_with(" ") => {
                let content = &line[4..];
                match content {
                    // 字段
                    content if !content.contains("(") => {
                        let field = content.split(" ").collect::<Vec<&str>>();
                        let original = field[1];
                        let obfuscated = field[3];
                        let field_type = field[0];
                        //conn.execute("INSERT OR REPLACE INTO vanilla_fields (class_id, original, obfuscated, field_type) VALUES (?1, ?2, ?3, ?4)", (current_class_id, &original, &obfuscated, &field_type))?;
                        field_pre.execute((current_class_id, &original, &obfuscated, &field_type))?;
                        info!("Processed field(ID: {}): {} ->  {}", conn.last_insert_rowid(), original, obfuscated);
                    },
                    // 方法
                    content if content.contains("(") => {
                        let _parameters = content.split('(').nth(1).and_then(|s| s.split(')').next()).unwrap_or("");
                        let parameter_types = serde_json::to_string(&_parameters.split(',').map(|p| p.trim()).collect::<Vec<_>>())?;
                        let original = content.split(' ').nth(1).and_then(|part| part.split('(').next()).unwrap_or("").trim();
                        let obfuscated = content.split(' ').nth(3).unwrap_or("").trim();
                        let return_type = content.split(':').nth(2).and_then(|part| part.split(' ').next()).unwrap_or("").trim();
                        let src = &current_metadata_source_file;
                        match state {
                            State::Class => {method_pre.execute((current_class_id, &original, &obfuscated, &return_type, &parameter_types))?;}//{conn.execute("INSERT OR REPLACE INTO vanilla_methods (class_id, original, obfuscated, return_type, parameter_types) VALUES (?1, ?2, ?3, ?4, ?5)", (current_class_id, &original, &obfuscated, &return_type, &parameter_types))?;}
                            State::Metadata => {method_meta_pre.execute((src, &original, &obfuscated, &return_type, &parameter_types))?;}//{conn.execute("INSERT OR REPLACE INTO vanilla_methods (source_file, original, obfuscated, return_type, parameter_types) VALUES (?1, ?2, ?3, ?4, ?5)", (src, &original, &obfuscated, &return_type, &parameter_types))?;}
                            State::None => error!("Error state"),
                        }
                        info!("Processed method(ID: {}): {} ->  {}", conn.last_insert_rowid(), original, obfuscated);
                    }
                    _ => { error!("Error format {}", content); }
                }
            }
            _ => {
                error!("Error format {}", line);
            }
        }

    }
    conn.execute("COMMIT", [])?;
    Ok(line_len)
}
/*
 *
 *  * Created: 2026-2-27 0:17:46
 *  * File: srg.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::fs;
use std::io::BufRead;
use log::debug;
use rusqlite::Connection;

pub fn process_srg(input: &str, db: &str, version: &str) -> anyhow::Result<usize> {
    let file = fs::read_to_string(input)?;
    let lines = file.lines().collect::<Vec<_>>();
    let line_len = lines.len();
    let conn = Connection::open(db)?;
    conn.execute("PRAGMA synchronous = OFF", [])?;
    conn.execute("PRAGMA reverse_unordered_selects = 1", [])?;
    conn.execute("PRAGMA temp_store = MEMORY", [])?;
    conn.execute("BEGIN TRANSACTION", [])?;
    let mut class_pre = conn.prepare_cached("INSERT OR REPLACE INTO forge_classes (version, srg, named) VALUES (?1, ?2, ?3)")?;
    let mut method_pre = conn.prepare_cached("INSERT OR REPLACE INTO forge_methods (srg, desc, named) VALUES (?1, ?2, ?3)")?;
    let mut field_pre = conn.prepare_cached("INSERT OR REPLACE INTO forge_fields (srg, named) VALUES (?1, ?2)")?;
    for line in lines {
        let split = line.split_whitespace().collect::<Vec<_>>();

        match split[0] {
            // 类
            "CL:" => {
                class_pre.execute((version, split[1], split[2]))?;
                debug!("Processed class(ID: {}) : {} -> {}", conn.last_insert_rowid(), split[1], split[2])
            }
            // 方法
            "MD:" => {
                method_pre.execute((split[1], split[2], split[3]))?; // 实际上4也是desc
                debug!("Processed method(ID: {}) : {} -> {}", conn.last_insert_rowid(), split[1], split[3])
            }
            // 字段
            "FD:" => {
                field_pre.execute((split[1], split[2]))?;
                debug!("Processed field(ID: {}) : {} -> {}", conn.last_insert_rowid(), split[1], split[2])
            }
            _ => {}
        }
    }
    conn.execute("COMMIT;", [])?;
    Ok(line_len)
}
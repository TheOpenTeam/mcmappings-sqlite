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
use std::thread::current;
use log::info;
use rusqlite::Connection;
#[derive(Debug, PartialEq)]
enum State {
    Metadata,
    Class,
    None,
}

pub fn process_proguard(path: &str, db: &str, version: &str) -> anyhow::Result<()> {
    let conn = Connection::open(db)?;
    let file = fs::read_to_string(path)?;
    let mut lines: Vec<&str> = file
        .lines()
        .filter(|line| {
        let trimmed = line.trim_start();
        !trimmed.is_empty() && !trimmed.starts_with('#')
    })
        .collect(); // 去掉元数据和空行
    lines.remove(0); // 删除第一行的微软注释

    let mut current_class_obfuscated: Option<String> = None;
    let mut current_class_original: Option<String> = None;
    let mut current_class_id = 0;
    let mut state = State::None;
    for line in lines {
        match line {
            // 不以空格开头则为类
            line if !line.starts_with(" ") => {
                state = State::Class;
                let class = line.trim().split(" ").collect::<Vec<&str>>();
                current_class_obfuscated = Some(class[2].replace(":", ""));
                current_class_original = Some(class[0].to_string());
                conn.execute("INSERT INTO vanilla_classes (version, original, obfuscated) VALUES (?1, ?2, ?3)", (version, &current_class_original, &current_class_obfuscated))?;
                current_class_id = conn.last_insert_rowid();
                info!("Processed class(ID: {}), : {:?} ->  {:?}", current_class_id, current_class_original, current_class_obfuscated);
            }
            // 元数据，标记状态
            line if line.starts_with("#") => state = State::Metadata,
            // 类字段与方法
            line if line.starts_with(" ") && state == State::Class => {
                let content = &line[4..];
                match content {
                    // 字段
                    content if !content.contains("(") => {
                        let field = content.split(" ").collect::<Vec<&str>>();
                        let original = field[1];
                        let obfuscated = field[3];
                        let field_type = field[0];
                        conn.execute("INSERT INTO vanilla_fields (class_id, original, obfuscated, field_type) VALUES (?1, ?2, ?3, ?4)", (current_class_id, &original, &obfuscated, &field_type))?;
                        info!("Processed field: {} ->  {}", original, obfuscated);
                    },
                    _ => {println!("{}", line)}
                }

            }
            _ => {println!("{}", line)}
        }
    }

    Ok(())
}
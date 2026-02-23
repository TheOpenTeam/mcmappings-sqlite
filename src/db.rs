/*
 *
 *  * Created: 2026-2-22 3:45:21
 *  * File: db.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::fs;
use std::path::Path;
use log::{info, warn};
use rusqlite::Connection;

pub(crate) fn create_empty_db(path: &str) -> Result<(), anyhow::Error> {
    if Path::new(path).exists() {
        warn!("{} already exists, removed.", path);
        fs::remove_file(path)?;
    }
    let conn = Connection::open(path)?;
    info!("Creating an empty database...");
    // vanilla
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vanilla_classes (
            id INTEGER PRIMARY KEY,
            version TEXT NOT NULL,
            original TEXT NOT NULL,
            obf_class TEXT NOT NULL,
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vanilla_methods (
            id INTEGER PRIMARY KEY,
            class_id INTEGER NOT NULL,
            readable_method TEXT NOT NULL,
            obf_method TEXT NOT NULL,
            descriptor TEXT NOT NULL,
            start_line INTEGER,
            end_line INTEGER,
            FOREIGN KEY(class_id) REFERENCES vanilla_classes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vanilla_fields (
            id INTEGER PRIMARY KEY,
            class_id INTEGER NOT NULL,
            readable_field TEXT NOT NULL,
            obf_field TEXT NOT NULL,
            field_type TEXT,
            FOREIGN KEY(class_id) REFERENCES vanilla_classes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // fabric
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_classes (
            id INTEGER PRIMARY KEY,
            version TEXT NOT NULL,
            readable_class TEXT NOT NULL,
            intermediary_class TEXT NOT NULL,
            obf_class TEXT NOT NULL,
            source_file TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_methods (
            id INTEGER PRIMARY KEY,
            class_id INTEGER NOT NULL,
            readable_method TEXT NOT NULL,
            intermediary_method TEXT NOT NULL,
            obf_method TEXT NOT NULL,
            descriptor TEXT NOT NULL,
            FOREIGN KEY(class_id) REFERENCES fabric_classes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_fields (
            id INTEGER PRIMARY KEY,
            class_id INTEGER NOT NULL,
            readable_field TEXT NOT NULL,
            intermediary_field TEXT NOT NULL,
            obf_field TEXT NOT NULL,
            field_type TEXT,
            FOREIGN KEY(class_id) REFERENCES fabric_classes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // forge
    conn.execute(
        "CREATE TABLE IF NOT EXISTS forge_classes (
            id INTEGER PRIMARY KEY,
            version TEXT NOT NULL,
            readable_class TEXT NOT NULL,
            srg_class TEXT NOT NULL,
            obf_class TEXT NOT NULL,
            source_file TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS forge_methods (
            id INTEGER PRIMARY KEY,
            class_id INTEGER NOT NULL,
            readable_method TEXT NOT NULL,
            srg_method TEXT NOT NULL,
            obf_method TEXT NOT NULL,
            descriptor TEXT NOT NULL,
            FOREIGN KEY(class_id) REFERENCES forge_classes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS forge_fields (
            id INTEGER PRIMARY KEY,
            class_id INTEGER NOT NULL,
            readable_field TEXT NOT NULL,
            srg_field TEXT NOT NULL,
            obf_field TEXT NOT NULL,
            field_type TEXT,
            FOREIGN KEY(class_id) REFERENCES forge_classes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vanilla_classes_obf ON vanilla_classes(obf_class)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vanilla_methods_obf ON vanilla_methods(obf_method)", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_classes_obf ON fabric_classes(obf_class)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_methods_obf ON fabric_methods(obf_method)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_classes_intermediary ON fabric_classes(intermediary_class)", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_forge_classes_obf ON forge_classes(obf_class)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_forge_methods_obf ON forge_methods(obf_method)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_forge_classes_srg ON forge_classes(srg_class)", [])?;
    info!("Database created successfully");
    Ok(())

}

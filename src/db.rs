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
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    info!("Creating an empty database...");
    // vanilla
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vanilla_classes (
            class_id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL,
            original TEXT NOT NULL,
            obfuscated TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vanilla_methods (
            method_id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER,
            source_file TEXT,
            original TEXT NOT NULL,
            obfuscated TEXT NOT NULL,
            return_type TEXT NOT NULL,
            parameter_types TEXT,
            FOREIGN KEY(class_id) REFERENCES vanilla_classes(class_id) ON DELETE CASCADE
        )", // parameter_types存的是json，对于某些“顶级方法”，无classid而有source_file
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS vanilla_fields (
        field_id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER NOT NULL,
            original TEXT NOT NULL,
            obfuscated TEXT NOT NULL,
            field_type TEXT,
            FOREIGN KEY(class_id) REFERENCES vanilla_classes(class_id) ON DELETE CASCADE
        )",
        [],
    )?;

    // fabric tiny v1
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_classes_v1 (
            class_id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL,
            named TEXT NOT NULL,
            intermediary TEXT NOT NULL,
            official TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_methods_v1 (
            method_id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER NOT NULL,
            parent_class TEXT NOT NULL,
            desc TEXT NOT NULL,
            official TEXT NOT NULL,
            named TEXT NOT NULL,
            intermediary TEXT NOT NULL,
            FOREIGN KEY(class_id) REFERENCES fabric_classes_v1(class_id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_fields_v1 (
            field_id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER NOT NULL,
            parent_class TEXT NOT NULL,
            field_type TEXT NOT NULL,
            official TEXT NOT NULL,
            named TEXT NOT NULL,
            intermediary TEXT NOT NULL,
            FOREIGN KEY(class_id) REFERENCES fabric_classes_v1(class_id) ON DELETE CASCADE
        )",
        [],
    )?;
    // fabric tiny v2
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_classes_v2 (
            class_id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL,
            intermediary TEXT NOT NULL,
            named TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_methods_v2 (
            method_id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER NOT NULL,
            desc TEXT NOT NULL,
            intermediary TEXT NOT NULL,
            named TEXT NOT NULL,
            parameters TEXT,
            FOREIGN KEY(class_id) REFERENCES fabric_classes_v2(class_id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS fabric_fields_v2 (
            field_id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER NOT NULL,
            field_type TEXT NOT NULL,
            intermediary TEXT NOT NULL,
            named TEXT NOT NULL,
            FOREIGN KEY(class_id) REFERENCES fabric_classes_v2(class_id) ON DELETE CASCADE
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
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vanilla_classes_obf ON vanilla_classes(obfuscated)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_vanilla_methods_obf ON vanilla_methods(obfuscated)", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_classes_v1_obf ON fabric_classes_v1(official)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_methods_obf ON fabric_methods_v1(official)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_classes_v1_intermediary ON fabric_classes_v1(intermediary)", [])?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_classes_v2_obf ON fabric_classes_v2(named)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_fabric_classes_v2_intermediary ON fabric_classes_v2(intermediary)", [])?;


    conn.execute("CREATE INDEX IF NOT EXISTS idx_forge_classes_obf ON forge_classes(obf_class)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_forge_methods_obf ON forge_methods(obf_method)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_forge_classes_srg ON forge_classes(srg_class)", [])?;
    info!("Database created successfully");
    Ok(())

}

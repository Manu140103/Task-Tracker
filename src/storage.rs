// src/storage.rs
use std::fs::{File};
use std::io::{Read, Write};
use std::path::Path;
use std::process;

use crate::task::Task;

const FILE_NAME: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE_NAME).exists() {
        File::create(FILE_NAME).expect("No se pudo crear el archivo");
        return Vec::new();
    }

    let mut file = File::open(FILE_NAME).expect("No se pudo abrir el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error al leer el archivo");

    if contents.trim().is_empty() {
        return Vec::new();
    }

    serde_json::from_str(&contents).unwrap_or_else(|_| {
        eprintln!("Error al parsear JSON");
        process::exit(1);
    })
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let mut file = File::create(FILE_NAME).expect("No se pudo abrir el archivo");
    let data = serde_json::to_string_pretty(tasks).expect("Error al serializar");
    file.write_all(data.as_bytes()).expect("Error al escribir archivo");
}

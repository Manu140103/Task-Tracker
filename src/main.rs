// src/main.rs
mod task;
mod storage;

use std::env;
use std::process;

use crate::task::Task;
use crate::storage::{load_tasks, save_tasks};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Por favor, especifique una acción: agr, act, eli, eestatus, lista");
        process::exit(1);
    }

    let action = &args[1];

    let mut tasks = load_tasks();

    match action.as_str() {
        "agr" => {
            if args.len() < 3 {
                eprintln!("Uso: agr <descripcion>");
                process::exit(1);
            }
            let description = args[2..].join(" ");
            add_task(&mut tasks, description);
        }
        "act" => {
            if args.len() < 4 {
                eprintln!("Uso: act <id> <descripcion>");
                process::exit(1);
            }
            let id: u32 = args[2].parse().unwrap_or_else(|_| {
                eprintln!("ID inválido");
                process::exit(1);
            });
            let description = args[3..].join(" ");
            update_task(&mut tasks, id, description);
        }
        "eli" => {
            if args.len() < 3 {
                eprintln!("Uso: eli <id>");
                process::exit(1);
            }
            let id: u32 = args[2].parse().unwrap_or_else(|_| {
                eprintln!("ID inválido");
                process::exit(1);
            });
            delete_task(&mut tasks, id);
        }
        "estatus" => {
            if args.len() < 4 {
                eprintln!("Uso: estatus <id> <en_espera|en_progeso|listo>");
                process::exit(1);
            }
            let id: u32 = args[2].parse().unwrap_or_else(|_| {
                eprintln!("ID inválido");
                process::exit(1);
            });
            let status = &args[3];
            change_status(&mut tasks, id, status);
        }
        "lista" => {
            let filter = if args.len() >= 3 { &args[2] } else { "todo" };
            list_tasks(&tasks, filter);
        }
        _ => {
            eprintln!("Acción desconocida, por favor use: agr, act, eli, eestatus, lista ");
            process::exit(1);
        }
    }

    save_tasks(&tasks);
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task::new(id, description));
    println!("Tarea agregada con ID {}", id);
}

fn update_task(tasks: &mut Vec<Task>, id: u32, description: String) {
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.description = description;
            println!("Tarea {} actualizada", id);
        }
        None => {
            eprintln!("Tarea no encontrada");
        }
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    let initial_len = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() == initial_len {
        eprintln!("Tarea no encontrada");
    } else {
        println!("Tarea {} eliminada", id);
    }
}

fn change_status(tasks: &mut Vec<Task>, id: u32, status: &str) {
    let valid_status = ["en_espera", "en_progeso", "listo"];
    if !valid_status.contains(&status) {
        eprintln!("Estado inválido. Use: en_espera, en_progeso, listo");
        return;
    }

    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.status = status.to_string();
            println!("Estado de tarea {} cambiado a {}", id, status);
        }
        None => {
            eprintln!("Tarea no encontrada");
        }
    }
}

fn list_tasks(tasks: &Vec<Task>, filter: &str) {
    let filtered_tasks: Vec<&Task> = match filter {
        "en_espera" => tasks.iter().filter(|t| t.status == "en_espera").collect(),
        "en_progeso" => tasks.iter().filter(|t| t.status == "en_progeso").collect(),
        "listo" => tasks.iter().filter(|t| t.status == "listo").collect(),
        "todo" => tasks.iter().collect(),
        _ => {
            eprintln!("Filtro desconocido. Use: todo, en_espera, en_progeso, listo");
            return;
        }
    };

    if filtered_tasks.is_empty() {
        println!("No hay tareas para mostrar");
    } else {
        for task in filtered_tasks {
            println!("ID: {} | {} | Estado: {}", task.id, task.description, task.status);
        }
    }
}

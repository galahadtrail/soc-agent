pub mod cheching;
pub mod connection;
pub mod greetings;
pub mod logger;

use cheching::{matching_rules, write_hash_rules_from_file};
use connection::connect;
use greetings::print_hello_message;

use crate::logger::write_current_dt_to_log;
use colored::*;
use notify::event::{CreateKind, ModifyKind};
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Result, Watcher};
use std::fs::canonicalize;
use std::io;
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    print_hello_message();
    let _ = write_current_dt_to_log("logs/power.log", "success", "program runs!");

    let alerts: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let alerts_sendable: Arc<Mutex<Vec<String>>> = Arc::clone(&alerts);
    let alerts_clone_wr = Arc::clone(&alerts);

    ctrlc::set_handler(move || {
        println!("Получен сигнал Ctrl+C! Выход из функции слежения");
        let alerts_clone = Arc::clone(&alerts);
        connect(alerts_clone);
        std::process::exit(0);
    })
    .expect("Ошибка при установке обработчика Ctrl+C");

    let new_rules_for_me = connect(alerts_sendable);

    let new_rules: Vec<String> = new_rules_for_me
        .split('@') // Используем split для разделения по запятой
        .map(String::from)
        .collect();
    let rules = new_rules.clone();
    let _ = write_hash_rules_from_file("src/rules/rules.txt", new_rules);

    // Создаем канал для получения событий
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    // Создаем объект watcher с задержкой 2 секунды
    let mut watcher = recommended_watcher(tx)?;

    let mut path = String::new();
    let write = "Enter the path where you want to track files:"
        .truecolor(193, 251, 222)
        .on_purple();
    println!("{}", write);
    let _ = io::stdin().read_line(&mut path);

    // Указываем директорию для отслеживания
    watcher.watch(Path::new(&path.trim()), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == EventKind::Create(CreateKind::File)
                    || event.kind
                        == EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any))
                {
                    for path in event.paths.iter() {
                        let math_res = matching_rules(&rules, path.to_path_buf());
                        if math_res {
                            println!(
                                "Achtung! {}",
                                String::from(canonicalize(path).unwrap().to_str().unwrap())
                            );
                            let _ = write_current_dt_to_log(
                                "logs/alerts.log",
                                "success",
                                &format!(
                                    "Achtung! {}",
                                    String::from(canonicalize(path).unwrap().to_str().unwrap())
                                ),
                            );
                            alerts_clone_wr
                                .lock()
                                .unwrap()
                                .push(String::from(canonicalize(path).unwrap().to_str().unwrap()));
                        }
                    }
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }

    let _ = write_current_dt_to_log("logs/power.log", "success", "program exits!");
    Ok(())
}

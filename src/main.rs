pub mod cheching;
pub mod connection;
pub mod greetings;

use cheching::{matching_rules, write_hash_rules_from_file};
use connection::connect;
use greetings::print_hello_message;

use colored::*;
use notify::event::{CreateKind, ModifyKind};
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Result, Watcher};
use std::io;
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    print_hello_message();

    let alerts: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let alerts_clone = Arc::clone(&alerts);

    ctrlc::set_handler(move || {
        println!("Получен сигнал Ctrl+C! Выход из функции слежения");
        let alerts = alerts_clone.lock().unwrap();
        connect(&*alerts);
        std::process::exit(0);
    })
    .expect("Ошибка при установке обработчика Ctrl+C");

    let new_rules_for_me = connect(&*alerts.lock().unwrap());

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
                            alerts
                                .lock()
                                .unwrap()
                                .push(String::from(path.to_str().unwrap()));
                        }
                    }
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }

    Ok(())
}

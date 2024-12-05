pub mod cheching;
pub mod greetings;

use cheching::{cheching_files_hash, read_hash_rules_from_file, write_hash_rules_from_file};
use greetings::print_hello_message;

use notify::event::{CreateKind, ModifyKind};
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc;

fn main() -> Result<()> {
    print_hello_message();
    // Создаем канал для получения событий
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    // Создаем объект watcher с задержкой 2 секунды
    let mut watcher = recommended_watcher(tx)?;
    let mut rules = read_hash_rules_from_file("src/rules/rules.txt");

    // Указываем директорию для отслеживания
    watcher.watch(Path::new("."), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == EventKind::Create(CreateKind::File) {
                    println!("{:?}", event.paths);
                    for path in event.paths.iter() {
                        cheching_files_hash(path.to_path_buf());
                    }
                }
                if event.kind == EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any))
                {
                    for path in event.paths.iter() {
                        cheching_files_hash(path.to_path_buf());
                    }
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }

    Ok(())
}

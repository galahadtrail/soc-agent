pub mod cheching;
pub mod greetings;

use cheching::cheching_files_hash;
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

    // Указываем директорию для отслеживания
    watcher.watch(Path::new("."), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == EventKind::Create(CreateKind::File) {
                    println!("{:?}", event.paths);
                    for path in event.paths {
                        cheching_files_hash(path);
                    }
                }
                if event.kind == EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any))
                {
                    println!("1");
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }

    Ok(())
}

pub mod greetings;
use greetings::print_hello_message;

use notify::event::CreateKind;
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
                    println!("!")
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }

    Ok(())
}

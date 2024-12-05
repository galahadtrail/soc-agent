pub mod cheching;
pub mod greetings;

use cheching::{cheching_files_hash, write_hash_rules_from_file};
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
    let str1 = String::from("92dad9443e4dd6d70a7f11872101ebff87e21798e4fbb26fa4bf590eb440e71b");
    let str2 = String::from("a03ab19b866fc585b5cb1812a2f63ca861e7e7643ee5d43fd7106b623725fd67");
    let vec = vec![str1, str2];
    write_hash_rules_from_file("src/rules/rules.txt", vec);

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

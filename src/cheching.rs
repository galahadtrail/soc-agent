use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

pub fn write_hash_rules_from_file(path_to_file: &str, rules: Vec<String>) -> std::io::Result<()> {
    let rules_jsoned: Vec<String> = rules
        .iter()
        .map(|rule| serde_json::to_string(rule).unwrap())
        .collect();

    let file = File::create(path_to_file)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &rules_jsoned)?;
    writer.flush()?;

    Ok(())
}

pub fn matching_rules(rules: &Vec<String>, path_to_file: PathBuf) -> bool {
    let hash_alert = cheching_files_hash(path_to_file).unwrap();
    for rule in rules.iter() {
        if *rule == hash_alert {
            return true;
        }
    }
    false
}

fn cheching_files_hash(path_to_file: PathBuf) -> io::Result<String> {
    // Открываем файл для чтения
    let file = File::open(path_to_file)?;
    let mut reader = BufReader::new(file);

    // Создаем объект хеширования SHA3-256
    let mut hasher = Sha3_256::new();

    // Читаем файл по частям и обновляем хеш
    let mut buffer = [0; 1024]; // Буфер для чтения
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // Конец файла
        }
        hasher.update(&buffer[..bytes_read].trim_ascii()); // Обновляем хеш
    }

    // Получаем финальный хеш
    let result = hasher.finalize();

    let param = format!("{:x}", result);

    Ok(param)
}

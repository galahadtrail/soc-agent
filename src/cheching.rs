use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

pub fn cheching_files_hash(path_to_file: PathBuf) -> io::Result<()> {
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
        hasher.update(&buffer[..bytes_read]); // Обновляем хеш
    }

    // Получаем финальный хеш
    let result = hasher.finalize();

    // Выводим результат в шестнадцатеричном формате
    println!("SHA3-256 hash: {:x}", result);

    Ok(())
}

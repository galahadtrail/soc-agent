use chrono::{DateTime, Utc};
use std::fs::OpenOptions;
use std::io::{self, Write};

fn gimme_date_and_time() -> DateTime<Utc> {
    let current_date_time = Utc::now();
    current_date_time
}

pub fn write_current_dt_to_log(path: &str, success: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true) // Добавление данных в конец файла
        .open(path)?;

    let curr_dt = format!("{}", gimme_date_and_time());
    let result = format!(
        "DT: {} ID: {} SUCCESS: {} CONTENT: {}",
        curr_dt, 0, success, content
    );

    file.write_all(result.as_bytes())?;
    Ok(())
}

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::{Arc, Mutex};

pub fn connect(alerts: Arc<Mutex<Vec<String>>>) -> String {
    // Подключаемся к серверу
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // Читаем сообщение от сервера
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let msg = str::from_utf8(&buffer[..bytes_read]).unwrap();

    let new_rules_for_me = String::from(msg);

    // Отправляем ответ серверу
    let mut response = String::new();

    if alerts.lock().unwrap().len() != 0 {
        response = alerts.lock().unwrap().join("@");
    } else {
        response = "Empty".to_string();
    }

    stream.write(response.as_bytes()).unwrap();

    new_rules_for_me
}

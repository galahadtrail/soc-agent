use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub fn connect() {
    // Подключаемся к серверу
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // Читаем сообщение от сервера
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let msg = str::from_utf8(&buffer[..bytes_read]).unwrap();
    println!("Received from server: {}", msg);

    // Отправляем ответ серверу
    let response = "Hello from client!";
    stream.write(response.as_bytes()).unwrap();
}

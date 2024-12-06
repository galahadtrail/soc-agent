use std::io::{self, BufRead};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
pub async fn connect() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = socket.into_split();

    // Чтение сообщений от сервера
    tokio::spawn(async move {
        let mut reader = tokio::io::BufReader::new(reader);
        let mut buf = String::new();

        loop {
            buf.clear();
            if reader.read_line(&mut buf).await.unwrap() == 0 {
                break; // Соединение закрыто
            }
            println!("Получено сообщение: {}", buf);
        }
    });

    // Отправка сообщений на сервер
    let stdin = io::stdin();
    let mut stdin_reader = stdin.lock();

    loop {
        let mut input = String::new();
        stdin_reader.read_line(&mut input).unwrap();
        writer.write_all(input.as_bytes()).await.unwrap();
    }
}

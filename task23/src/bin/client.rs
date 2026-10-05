use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // 1. Подключаемся к серверу
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Клиент подключился!");

    loop {
        let mut req = String::new();
        io::stdin()
            .read_line(&mut req)
            .expect("Failed to read line");
        // 2. Отправляем данные на сервер
        stream.write_all(req.as_bytes())?;

        // 3. [БЛОКИРОВКА КЛИЕНТА] Замираем на этой строчке, пока сервер не ответит
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;

        // 4. Читаем то, что прислал сервер
        let res = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("STATUS: {}", res);

        thread::sleep(Duration::from_secs(2)); // Пауза перед следующим шагом
    }

    Ok(())
}

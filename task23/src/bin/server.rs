use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
fn handle_client(mut stream: TcpStream, db: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = [0; 512];
    let mut pending = String::new();
    loop {
        // [БЛОКИРОВКА СЕРВЕРА] Ждем, пока клиент что-то пришлет
        match stream.read(&mut buffer) {
            Ok(0) => break, // Клиент закрыл соединение
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                pending.push_str(&msg);
                while let Some(pos) = pending.find('\n') {
                    let command = pending[..pos].trim().to_string();
                    pending = pending[pos + 1..].to_string();

                    let parts: Vec<&str> =
                        command.splitn(3, " ").collect();
                    let response = match parts[0] {
                        "SET" => {
                            let key = parts[1].to_string();
                            let value = parts[2].to_string();

                            let mut map = db.lock().unwrap();
                            map.insert(key.clone(), value);
                            format!("OK: Записано [{}]\n", key)
                        }
                        "GET" => {
                            let key = parts[1].to_string();
                            let map = db.lock().unwrap();
                            match map.get(&key) {
                                Some(val) => format!("OK: [{}]\n", val),
                                None => format!("Nil"),
                            }
                        }
                        "DEL" => {
                            let key = parts[1].to_string();
                            let mut map = db.lock().unwrap();
                            match map.remove(&key) {
                                Some(val) => format!("REMOVED: [{}]\n", val),
                                None => format!("Nil"),
                            }
                        }
                        _ => "ERR: Неизвестная команда. Используйте: SET <k> <v> или GET <k>\n"
                            .to_string(),
                    };
                    if stream.write_all(response.as_bytes()).is_err() {
                        break;
                    }
                }


            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let db = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let db_clone = Arc::clone(&db);
            thread::spawn(|| handle_client(stream, db_clone)); // Каждому клиенту — свой поток
        }
    }
    Ok(())
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// تابع برای پردازش هر کلاینت
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // اتصال بسته شده است
                break;
            }
            Ok(n) => {
                // داده دریافت شده و بازتاب داده می‌شود (echo)
                stream.write_all(&buffer[0..n]).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    // سرور TCP که روی پورت 7878 گوش می‌دهد
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on 127.0.0.1:7878");

    // پذیرش کلاینت‌ها به صورت حلقه بی‌نهایت
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                // برای هر کلاینت جدید یک thread جدید ایجاد می‌شود
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

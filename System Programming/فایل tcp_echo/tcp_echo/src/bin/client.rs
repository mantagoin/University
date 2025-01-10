use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    println!("Connected to the server!");

    let mut input = String::new();
    
    loop {
        input.clear();
        print!("Enter message: ");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();
        
        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        println!("Server response: {}", String::from_utf8_lossy(&buffer[..n]));
    }
}

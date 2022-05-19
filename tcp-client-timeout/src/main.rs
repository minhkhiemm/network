use std::fs::read;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::from_utf8;
use std::time::Duration;

fn main() {
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1))
        .expect("Could not connect to server");
    stream.set_read_timeout(Some(Duration::from_secs(6))).expect("Could not set timeout");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input from Stdin");
        stream.write(input.as_bytes()).expect("Failed to write to server");
        
        let mut buffer: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        print!("{}", from_utf8(&buffer).expect("Could write buffer as string"))
    }
}

use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn client(target: &str) {
    let output = "GET / HTTP/1.1\n";

    loop {
        let mut stream = TcpStream::connect(target).expect("client cannot connect");

        eprintln!("Connection established!");
        stream.write(output.as_bytes()).unwrap();
        stream.flush().unwrap();
        eprintln!("Sent request!");
        stream.write_all(output.as_bytes()).unwrap();

        let mut client_buffer = [0u8; 1024];
        loop {
            match stream.read(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    } else {
                        io::stdout().write(&client_buffer).unwrap();
                        io::stdout().flush().unwrap();
                    }
                }
                Err(error) => panic!(error.to_string()),
            }
        }
    }
}

use std::net::TcpStream;
use std::io::{self, Read, Write};

pub fn client() {
    let output = "GET / HTTP/1.1\n";

    loop {
        let mut stream = TcpStream::connect("127.0.0.1:7878").expect("client cannot connect");

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
                },
                Err(error) => panic!(error.to_string()),
            }
        }

    }
}

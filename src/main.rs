mod thread_pool;
mod req;

use std::io::prelude::{Read, Write};
const HELLO: &str = "files/hello.html";
const BYE: &str = "files/bye.html";

pub fn main() {
    let mut shared = Shared::default();
    shared.load_file(HELLO);
    shared.load_file(BYE);
    let stdout = std::fs::File::create("/tmp/daemon.out").expect("Out file");
    let stderr = std::fs::File::create("/tmp/daemon.err").expect("Err file");
    daemonize::Daemonize::new()
        .pid_file("/tmp/daemon.pid")
        .chown_pid_file(true)
        .stdout(stdout)
        .stderr(stderr)
        .start().expect("Error");
    let shared = std::sync::Arc::new(shared);
    let pool = thread_pool::ThreadPool::new(4);
    for stream in std::net::TcpListener::bind("127.0.0.1:7878").expect("Bind").incoming() {
        let local_shared = std::sync::Arc::clone(&shared);
        pool.execute(|| { handle_connection(local_shared, stream.unwrap()) });
    }
}

#[derive(Default)]
struct Shared { cache: std::collections::HashMap<String, String> }
impl Shared {
    pub fn load_file(&mut self, file: &str) {
        self.cache.insert(file.to_string(), std::fs::read_to_string(file).expect(format!("Load file: {}", file).as_str()));
    }
    pub fn get_file(&self, file: &str) -> String {
        self.cache.get(file).map(|x|x.clone()).expect(format!("File {} was not pre loaded", file).as_str())
    }
}

fn handle_connection(shared: std::sync::Arc<Shared>, mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = req::Req::from_string(String::from_utf8_lossy(&buffer[..]).to_string());
    let contents = shared.get_file(if request.path == "/" { HELLO } else { BYE });
    let response = format!( "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
    stream.write_all(response.as_bytes()).unwrap();
    stream.shutdown(std::net::Shutdown::Both).expect("closing socket");
}

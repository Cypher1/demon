use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};

use std::collections::HashMap;
use std::fs;
use std::fs::File;

use std::sync::Arc;

use crate::thread_pool::{ThreadPool};
use crate::req::Req;

use daemonize::Daemonize;

pub fn server(target: &str) {
    let stdout = File::create("demon.out").unwrap();
    let stderr = File::create("demon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("demon.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        // .working_directory("/tmp") // for default behaviour.
        // .user("nobody")
        // .group("daemon") // Group name
        // .group(2)        // or group id.
        // .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| eprintln!("Daemon started?"))
        .privileged_action(|| "Executed before drop privileges");

    let mut shared = Shared::default();
    shared.load_file(HELLO);
    shared.load_file(BYE);

    match daemonize.start() {
        Ok(_) => {
            eprintln!("Success, daemonized");
            start(shared, target);
            eprintln!("Done");
        },
        Err(e) => eprintln!("Error, {}", e),
    }
}

const HELLO: &str = "files/hello.html";
const BYE: &str = "files/bye.html";

fn start(shared: Shared, target: &str) {
    let listener = TcpListener::bind(target).unwrap();
    let pool = ThreadPool::new(4);

    let shared = Arc::new(shared);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let local_shared = Arc::clone(&shared);

        pool.execute(|| {
            handle_connection(local_shared, stream);
        });
    }
}

#[derive(Default)]
struct Shared {
    cache: HashMap<String, String>
}

impl Shared {
    pub fn load_file(&mut self, file: &str) -> String {
        let contents = fs::read_to_string(file).unwrap();
        self.cache.insert(file.to_string(), contents.clone());
        contents
    }
    pub fn get_file(&self, file: &str) -> String {
        self.cache.get(file).map(|x|x.clone()).expect(format!("File {} was not pre loaded", file).as_str())
    }
}

fn handle_connection(shared: Arc<Shared>, mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..]);
    let request = Req::from_string(request_str.to_string());

    let contents = if request.path == "/" {
        shared.get_file(HELLO)
    } else {
        shared.get_file(BYE)
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Both).expect("closing socket");
}

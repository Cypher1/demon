mod client;
mod req;
mod server;
mod thread_pool;

use client::client;
use server::server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut is_daemon = false;

    for arg in args.iter() {
        if arg == "-d" {
            is_daemon = true;
        }
    }
    let target = "127.0.0.1:7878";
    if is_daemon {
        server(target);
    } else {
        client(target);
    }
}

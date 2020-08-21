use std::env;

mod server;
mod client;

mod thread_pool;
mod req;

use server::server;
use client::client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_daemon = false;

    let mut num = 0;

    for arg in args.iter() {
        if arg == "-d" {
            is_daemon = true;
        } else {
            num += 1;
        }
    }

    if is_daemon {
        server();
    } else {
        client();
    }
}

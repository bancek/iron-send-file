extern crate iron;
extern crate iron_send_file;

use std::path::Path;
use iron::prelude::*;
use iron_send_file::send_file;

fn main() {
    Iron::new(|req: &mut Request| {
            let path = Path::new("src/main.rs");

            let res = Response::new();

            send_file(req, res, path)
        })
        .http("localhost:3000")
        .unwrap();
}

mod server;
use server::*;

fn main() {
    match start_server() {
        Err(error) => panic!("Something went wrong: {}", error),
        Ok(()) => (),
    }
}

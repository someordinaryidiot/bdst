use std::env;

mod arg_handler;
mod dir_size;

fn main() {
    let args: Vec<String> = env::args().collect();

    arg_handler::handle(args);
}

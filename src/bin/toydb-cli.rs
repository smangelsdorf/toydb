use std::io::{stdin, stdout};

use toydb::cli::r#loop::cli_loop;

fn main() {
    println!("ToyDB CLI");

    cli_loop(stdin().lock(), stdout()).unwrap();
}

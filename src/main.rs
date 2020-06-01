extern crate rand;

use std::thread::spawn;

mod lib;
mod cube;

use lib::make_danger;
use lib::execute_danger;
use cube::cube;

fn main() -> Result<(), String> {
    make_danger();
    execute_danger();
    loop {
        spawn(cube).join().expect("thread panicked")?;
    }
}

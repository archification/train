extern crate rand;

use std::thread::spawn;

mod cube;

use cube::cube;

fn main() -> Result<(), String> {
    loop {
        spawn(cube).join().expect("thread panicked")?;
    }
}

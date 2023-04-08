use scoper::scoper::{OutputMode, ScoperMode, Summary};

use std::path::PathBuf;

mod error;
mod scoper;
use scoper::scoper::Scoper;
mod function;
mod printer;
mod processor;
mod utils;

fn main() {
    let dir = PathBuf::from("example"); //todo remove hardcoding

    let scoper = Scoper::new(dir);
    scoper.run();
}

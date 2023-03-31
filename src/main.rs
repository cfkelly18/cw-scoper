use scoper::scoper::{OutputMode, ScoperMode, Summary};

use std::path::PathBuf;

mod error;
mod scoper;
use scoper::scoper::Scoper;
mod printer;
mod processor;
mod utils;

fn main() {
    let dir = PathBuf::from("/home/colin/audit/cw-plus"); //todo remove hardcoding

    let scoper = Scoper::new(dir);
    scoper.run();
}

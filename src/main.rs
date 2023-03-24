use scoper::scoper::OutputMode;
use scoper::scoper::ScoperMode;
use std::path::Path;
use std::path::PathBuf;

mod error;
mod parser;
mod scoper;
use scoper::scoper::Scoper;
mod utils;
fn main() {
    let example = PathBuf::from("example/contract.rs");
    let example_vec = vec![example];
    let mode = ScoperMode::verbose;
    let output_mode: OutputMode = OutputMode::txt;

    let scoper = Scoper::new(example_vec, mode, output_mode);
    scoper.process();
}

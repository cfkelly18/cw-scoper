
use scoper::scoper::ScoperMode;
use scoper::scoper::OutputMode;
use std::path::Path;
use std::path::PathBuf;

mod scoper;
mod parser;
mod error;
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

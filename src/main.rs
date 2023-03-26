use scoper::scoper::OutputMode;
use scoper::scoper::ScoperMode;

use std::path::PathBuf;

mod error;
mod scoper;
use scoper::scoper::Scoper;
mod processor;
mod utils;

fn main() {
    let dir = PathBuf::from("example");
    let mode = ScoperMode::verbose;
    let output_mode: OutputMode = OutputMode::txt;
    let sorted_paths = utils::walk_dir(&dir); // todo handle errors better

    let scoper = Scoper::new(sorted_paths, mode, output_mode);
    scoper.process();
}

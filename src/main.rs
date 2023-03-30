use scoper::scoper::{ScoperMode, Summary,OutputMode};

use std::path::PathBuf;

mod error;
mod scoper;
use scoper::scoper::Scoper;
mod processor;
mod utils;
mod printer;

fn main() {
    let dir = PathBuf::from("/home/colin/audit/cw-plus");
    let mode = ScoperMode::verbose;
    let output_mode: OutputMode = OutputMode::txt;
    let sorted_paths = utils::walk_dir(&dir); // todo handle errors better

    let scoper = Scoper::new(sorted_paths, mode, output_mode);
    let summary: Summary = scoper.process();
    
}

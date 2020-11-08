// this should live in src/bin
// $ env RUST_LOG=output_log=info cargo run --bin output-log
use log::{info, warn};

pub fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing has been implemented");
}

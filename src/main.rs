pub mod app;
// pub mod back;
pub mod window;
use app::app::*;
use simple_logger::SimpleLogger;
#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

fn main() {
    let _logger = SimpleLogger::new();
    let _ = App::new();
}

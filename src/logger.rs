extern crate slog_term;
extern crate slog_stdlog;

use slog::DrainExt;
use ::slog::Logger;

pub fn setup () -> () {
    let drain = slog_term::streamer().compact().build().fuse();
    let logger = Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
    slog_stdlog::set_logger(logger).expect("Unable to setup the logger");
}

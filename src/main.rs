// cargo run -- -f minx.cfg

mod config;
use self::config::*;
mod logger;
use self::logger::*;



use std::{thread, time};



fn help () {
    println!("Usage: minx [-f json_file]");
    println!("Example:");
    println!("    minx -f config.cfg          # load config.cfg and running");
    println!("");
}

//#[async_std::main]
fn main() {
    let _args: Vec<String> = std::env::args ().collect ();
    //println!("{:?}", args);
    match get_config (&_args) {
        Some (_cfg) => {
            let _log = Logger::new (&_cfg.log_path);
        },
        None => help (),
    }
    loop {
        let _ten_s = time::Duration::from_secs(10);
        thread::sleep(_ten_s);
    }
}

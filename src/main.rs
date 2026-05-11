use std::env::args;

mod extend;
mod log;
mod single;

use extend::extend_mode;
use single::single_mode;

struct Config {
    command: String,
    parallel: bool,
    threads_number: Option<usize>,
    max_extend_length: usize,
    args: Vec<String>,
}

impl Config {
    fn new(args: impl Iterator<Item = String>) -> Self {
        let mut args = args;
        let mut command = String::new();
        let mut parallel = false;
        let mut threads_number: Option<usize> = None;
        let mut max_extend_length: usize = 256;
        for arg in &mut args {
            if arg.starts_with('-') {
                match arg.as_str() {
                    "-h" => {
                        print_help();
                        std::process::exit(0);
                    }
                    _ if arg.starts_with("-p") => {
                        parallel = true;
                        threads_number = arg[2..].parse::<usize>().ok();
                    }
                    _ if arg.starts_with("-m") => {
                        max_extend_length = arg[2..].parse::<usize>().unwrap_or(256);
                    }
                    _ => {}
                }
            } else {
                command = arg;
                break;
            }
        }
        Config {
            command,
            parallel,
            threads_number,
            max_extend_length,
            args: args.collect(),
        }
    }
}

fn main() {
    let mut args = args().skip(1);
    match args.next() {
        Some(arg) if arg == "s" => single_mode(Config::new(args)),
        Some(arg) if arg == "x" => extend_mode(Config::new(args)),
        Some(_) | None => {
            error!("Missing arguments.\n");
            print_help();
        }
    }
}

#[inline]
fn print_help() {
    info!(
        r"## Usage on Single mode
`gr s <options> <command including glob>`

### Options
-p (number of threads): Specify the number of threads to use for parallel execution. default: number of CPU threads


## Usage on Extend mode
`gr x <options> <command including glob>`

### Options
-p (number of threads): Specify the number of threads to use for parallel execution. default: number of CPU threads
-m (max depth): Specify the maximum extend glob. default: 256
"
    )
}

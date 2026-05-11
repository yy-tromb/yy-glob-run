use std::{env::args, process::exit};

mod extend;
mod log;
mod single;

use extend::extend_mode;
use single::single_mode;

#[derive(Clone)]
enum Label {
    Full,
    On,
    Off,
}

#[derive(Clone)]
struct Config {
    command: String,
    parallel: bool,
    threads_number: Option<usize>,
    max_extend_length: usize,
    stdout_label: Label,
    stderr_label: Label,
    ignore_error: bool,
    args: Vec<String>,
}

impl Config {
    fn new(args: impl Iterator<Item = String>) -> Self {
        let mut args = args;
        let mut config = Config {
            command: String::new(),
            parallel: false,
            threads_number: None,
            max_extend_length: 256,
            stdout_label: Label::On,
            stderr_label: Label::On,
            ignore_error: false,
            args: Vec::new(),
        };

        for arg in &mut args {
            if arg.starts_with('-') {
                match arg.as_str() {
                    "-h" => {
                        print_help();
                        std::process::exit(0);
                    }
                    _ if arg.starts_with("-p") => {
                        config.parallel = true;
                        config.threads_number = arg[2..].parse::<usize>().ok();
                    }
                    _ if arg.starts_with("-m") => {
                        config.max_extend_length = arg[2..].parse::<usize>().unwrap_or(256);
                    }
                    "-loutoff" => {
                        config.stdout_label = Label::Off;
                    }
                    "-loutfull" => {
                        config.stdout_label = Label::Full;
                    }
                    "-lerroff" => {
                        config.stderr_label = Label::Off;
                    }
                    "-lerrfull" => {
                        config.stderr_label = Label::Full;
                    }
                    "-e" => {
                        config.ignore_error = true;
                    }
                    _ if arg.starts_with("-") => {
                        error!("Unknown option: {}\n", arg);
                        print_help();
                        exit(1);
                    }
                    _ => {}
                }
            } else {
                config.command = arg;
                break;
            }
        }
        config.args = args.collect();
        config
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

#[inline]
fn label_to_string(label: &Label, path: &std::path::Path) -> String {
    match label {
        Label::Full => format!("[{:?}]", path),
        Label::On => {
            let mut comps = path.components().rev().take(2);
            let child = comps.next();
            let parent = comps.next();

            match (parent, child) {
                (Some(p), Some(c)) => {
                    // 親と子の両方がある：ここがメインルート
                    // format! に Cow を直接渡して 1回だけ String を作る
                    format!(
                        "[.../{}/{}]",
                        p.as_os_str().to_string_lossy(),
                        c.as_os_str().to_string_lossy()
                    )
                }
                (None, Some(c)) => {
                    // 子しかない（カレントディレクトリ直下）
                    // 1回だけコピーが発生（into_owned）
                    c.as_os_str().to_string_lossy().into_owned()
                }
                _ => String::new(),
            }
        }
        Label::Off => String::new(),
    }
}

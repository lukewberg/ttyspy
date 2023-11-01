use std::io;
use std::process::{Command, Stdio};

use libc;
use ttyspy::cmd_args::{self, Cli};

fn main() {
    // println!("Hello, world!");
    // libc::fopen(filename, mode)
    let stdin = io::stdin();
    let stderr = io::stderr();
    let stdout = io::stdout();
    let aws = std::env::var("");
    if let Some(args) = Cli::process() {
        let mut command = Command::new(args.program);
        command.args(args.program_args).status().expect("Failed to spawn command process!");
    }
    // Command::new(program);
    // stdin.lock();
    // stderr.lock();
    for line in stdin.lines() {
        println!("{}", line.expect("What the..."));
    }
}

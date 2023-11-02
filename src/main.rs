use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use clap::Parser;
use ttyspy::cli::Cli;

fn main() {
    // let stdin = io::stdin();
    // let stderr = io::stderr();
    // let stdout = io::stdout();
    // let aws = std::env::var("");
    let args = Cli::parse();
    let mut command = Command::new(args.program);
    let mut child = command
        .args(&(args.program_args.unwrap_or_default()))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn command process!");

    let (tx, rx) = mpsc::channel::<String>();
    let tx_err = tx.clone();

    let stdout = child
        .stdout
        .take()
        .expect("Unable to obtain child process stdout!");

    let stderr = child
        .stderr
        .take()
        .expect("Unable to obtain child process stdout!");

    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            // println!("{}", line.expect("What the..."));
            tx.send(format!(
                "{}",
                line.expect("Could not read line from stdout!")
            ))
            .expect("Message to main thread failed!");
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            // println!("{}", line.expect("What the..."));
            tx_err
                .send(format!(
                    "{}",
                    line.expect("Could not read line from stdout!")
                ))
                .expect("Message to main thread failed!");
        }
    });

    loop {
        match rx.recv() {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Cloud provider ttyspy will send logs to (defaults to AWS)
    #[arg(short, long, value_enum)]
    pub provider: Option<Providers>,
    /// Binary to capture logs from
    pub program: String,
    /// Arguments to pass to the captured binary
    #[arg(last = true)]
    pub program_args: Option<Vec<String>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Providers {
    /// Use for publishing logs to AWS SQS
    AWS,
    /// Use for publishing logs to GCP pub sub
    GCP,
}

// impl Cli {
//     pub fn process() -> Option<Cli> {
//         let args: Vec<String> = std::env::args().collect();
//         Some(Cli {
//             program: args[1].to_owned(),
//             program_args: args[2..].into(),
//             provider
//         })
//     }
// }

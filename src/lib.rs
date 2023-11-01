pub mod cmd_args {
    use clap::Parser;

    
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        pub program: String,
        pub program_args: Vec<String>,
    }

    impl Cli {
        pub fn process() -> Option<Cli> {
            let args: Vec<String> = std::env::args().collect();
            Some(Cli {
                program: args[1].to_owned(),
                program_args: args[2..].into(),
            })
        }
    }
}

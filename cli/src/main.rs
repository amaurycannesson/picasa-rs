pub mod cli;
mod cli_progress_reporter;

fn main() {
    if let Err(err) = cli::Cli::parse_args().run() {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}

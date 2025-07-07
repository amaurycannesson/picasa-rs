use dotenvy::dotenv;

pub mod cli;
mod cli_progress_reporter;

fn main() {
    dotenv().ok();

    if let Err(err) = cli::Cli::parse_args().run() {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}

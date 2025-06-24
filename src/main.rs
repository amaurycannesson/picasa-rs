use dotenvy::dotenv;

pub mod cli;

fn main() {
    dotenv().ok();

    cli::Cli::parse_args().run();
}

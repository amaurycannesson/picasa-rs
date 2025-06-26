use clap::{Parser, Subcommand};
use picasa_rs::{
    database,
    models::{
        geospatial_search_result::GeospatialSearchResult,
        semantic_search_result::SemanticSearchResult,
    },
    photo_repository::PgPhotoRepository,
    services::{
        geospatial_search::GeospatialSearchService, photo_embedder, photo_scanner,
        semantic_search::SemanticSearchService,
    },
    utils::progress_reporter,
};
use tabled::{Table, Tabled, settings::Style};

#[derive(Parser)]
#[command(name = "picasa-cli")]
#[command(about = "A photo scanning and management tool")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan photos in a directory
    Scan {
        /// The root directory path to scan for photos
        #[arg(help = "The root directory path to scan for photos")]
        root_directory: String,

        /// Extract EXIF data from photos (default: false)
        #[arg(long = "with-exif", help = "Enable EXIF data extraction")]
        with_exif: bool,

        /// Compute file hashes (default: false)
        #[arg(long = "with-hash", help = "Enable file hash computation")]
        with_hash: bool,
    },
    /// Process embeddings for photos
    Embed,
    /// Search for photos using semantic search
    Search {
        #[command(subcommand)]
        search_command: SearchCommands,
    },
}

#[derive(Subcommand)]
enum SearchCommands {
    /// Search for photos using a semantic query
    Semantic {
        /// The query string to search for
        #[arg(help = "The semantic query string to search for photos")]
        query: String,

        /// Optional similarity threshold (default: 0.0)
        #[arg(long = "threshold", help = "Similarity threshold for search results")]
        threshold: Option<f32>,

        /// Optional result limit (default: 10)
        #[arg(long = "limit", help = "Maximum number of results to return")]
        limit: Option<usize>,
    },
    /// Search for photos by country
    Geospatial {
        /// The country name to search for
        #[arg(help = "The country name to search for photos")]
        country_query: String,
    },
}

#[derive(Tabled)]
struct SemanticSearchResultRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Similarity")]
    pub similarity: f32,
}

impl From<SemanticSearchResult> for SemanticSearchResultRow {
    fn from(result: SemanticSearchResult) -> Self {
        Self {
            id: result.id,
            path: result.path,
            similarity: result.similarity,
        }
    }
}

#[derive(Tabled)]
struct GeospatialSearchResultRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Path")]
    pub path: String,
}

impl From<GeospatialSearchResult> for GeospatialSearchResultRow {
    fn from(result: GeospatialSearchResult) -> Self {
        Self {
            id: result.id,
            path: result.path,
        }
    }
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn run(self) {
        let pool = database::create_pool();
        let mut repo = PgPhotoRepository::new(pool);

        match self.command {
            Commands::Scan {
                root_directory,
                with_exif,
                with_hash,
            } => {
                let progress_reporter = progress_reporter::CliProgressReporter::new();
                photo_scanner::scan(
                    &root_directory,
                    &mut repo,
                    with_exif,
                    with_hash,
                    &progress_reporter,
                );
            }
            Commands::Embed => {
                let progress_reporter = progress_reporter::CliProgressReporter::new();
                photo_embedder::embed(&mut repo, &progress_reporter);
            }
            Commands::Search { search_command } => match search_command {
                SearchCommands::Semantic {
                    query,
                    threshold,
                    limit,
                } => {
                    let mut semantic_search_service = SemanticSearchService::new(
                        repo,
                        picasa_rs::services::embedders::text::TextEmbedder::new()
                            .expect("Failed to create embedder"),
                    );
                    let results = semantic_search_service.search(&query, threshold, limit);
                    let results_table: Vec<SemanticSearchResultRow> = results
                        .into_iter()
                        .map(SemanticSearchResultRow::from)
                        .collect();
                    let results_str = Table::new(results_table).with(Style::rounded()).to_string();
                    println!("{}", results_str);
                }
                SearchCommands::Geospatial { country_query } => {
                    let mut geospatial_search_service = GeospatialSearchService::new(repo);
                    let results = geospatial_search_service.search(&country_query);
                    let results_table: Vec<GeospatialSearchResultRow> = results
                        .into_iter()
                        .map(GeospatialSearchResultRow::from)
                        .collect();
                    let results_str = Table::new(results_table).with(Style::rounded()).to_string();
                    println!("{}", results_str);
                }
            },
        }
    }
}

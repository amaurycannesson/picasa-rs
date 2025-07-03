use anyhow::Result;
use clap::{Parser, Subcommand};
use picasa_rs::{
    database,
    models::Photo,
    repositories::{PgGeoRepository, PgPhotoRepository, face::repository::PgFaceRepository},
    services::{
        FaceDetectionService, PhotoEmbedderService, PhotoSearchParams, PhotoSearchService,
        embedders::{ClipImageEmbedder, ClipTextEmbedder},
        photo_scanner,
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
    /// Process embeddings for photos and faces
    #[command(subcommand)]
    Embed(EmbedCommands),
    /// Search photos
    Search {
        /// The query string to search for
        #[arg(long = "text", help = "The semantic query string to search for photos")]
        text: Option<String>,

        /// Optional similarity threshold (default: 0.0)
        #[arg(long = "threshold", help = "Similarity threshold for search results")]
        threshold: Option<f32>,

        /// Filter by country name
        #[arg(long = "country", help = "Filter photos by country name")]
        country: Option<String>,

        /// Filter by city name
        #[arg(long = "city", help = "Filter photos by city name")]
        city: Option<String>,

        /// Filter photos from this date onwards (ISO 8601 format)
        #[arg(
            long = "date-from",
            help = "Filter photos from this date onwards (e.g., 2023-01-01T00:00:00Z)"
        )]
        date_from: Option<String>,

        /// Filter photos up to this date (ISO 8601 format)
        #[arg(
            long = "date-to",
            help = "Filter photos up to this date (e.g., 2023-12-31T23:59:59Z)"
        )]
        date_to: Option<String>,

        /// Page number for pagination (default: 1)
        #[arg(
            long = "page",
            help = "Page number for pagination",
            default_value = "1"
        )]
        page: u32,

        /// Optional result limit (default: 10)
        #[arg(
            long = "per_page",
            help = "Maximum number of results per page",
            default_value = "10"
        )]
        per_page: u32,
    },
}

#[derive(Subcommand)]
enum EmbedCommands {
    /// Generate image embeddings for photos
    Image,
    /// Detect and embed faces in photos
    Face,
}

#[derive(Tabled)]
struct PhotoRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Path")]
    pub path: String,
}

impl From<Photo> for PhotoRow {
    fn from(result: Photo) -> Self {
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

    pub fn run(self) -> Result<()> {
        let pool = database::create_pool();

        let mut photo_repository = PgPhotoRepository::new(pool.clone());
        let geo_repository = PgGeoRepository::new(pool.clone());

        match self.command {
            Commands::Scan {
                root_directory,
                with_exif,
                with_hash,
            } => {
                let progress_reporter = progress_reporter::CliProgressReporter::new();

                let _ = photo_scanner::scan(
                    &root_directory,
                    &mut photo_repository,
                    with_exif,
                    with_hash,
                    &progress_reporter,
                );

                Ok(())
            }
            Commands::Embed(embed_command) => {
                match embed_command {
                    EmbedCommands::Image => {
                        let progress_reporter = progress_reporter::CliProgressReporter::new();
                        let image_embedder = ClipImageEmbedder::new()?;
                        let mut photo_embedder = PhotoEmbedderService::new(
                            photo_repository,
                            image_embedder,
                            progress_reporter,
                        );

                        photo_embedder.embed()?;
                    }
                    EmbedCommands::Face => {
                        let progress_reporter = progress_reporter::CliProgressReporter::new();
                        let face_repository = PgFaceRepository::new(pool);
                        let mut face_detection_service = FaceDetectionService::new(
                            photo_repository,
                            face_repository,
                            progress_reporter,
                        );

                        face_detection_service.detect_faces()?;
                    }
                }

                Ok(())
            }
            Commands::Search {
                text,
                threshold,
                country,
                city,
                date_from,
                date_to,
                page,
                per_page,
            } => {
                let text_embedder = ClipTextEmbedder::new()?;
                let mut photo_search =
                    PhotoSearchService::new(photo_repository, geo_repository, text_embedder);

                let search_params = PhotoSearchParams {
                    text,
                    threshold,
                    country,
                    city,
                    date_from,
                    date_to,
                    page,
                    per_page,
                };

                let result = photo_search.search(search_params)?;

                if result.items.is_empty() {
                    println!("No photos found matching the search criteria.");
                } else {
                    let photo_rows: Vec<PhotoRow> =
                        result.items.into_iter().map(|p| p.into()).collect();
                    let results_count = photo_rows.len();

                    let mut table = Table::new(photo_rows);
                    table.with(Style::rounded());
                    println!("{}", table);

                    let current_page = page;
                    let total_photos = result.total;
                    let total_pages = (total_photos as f64 / per_page as f64).ceil() as u32;

                    println!(
                        "\nPage {} of {} (showing {} photos, {} total)",
                        current_page, total_pages, results_count, total_photos
                    );
                }

                Ok(())
            }
        }
    }
}

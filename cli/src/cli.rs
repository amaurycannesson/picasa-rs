use anyhow::Result;
use clap::{Parser, Subcommand};
use picasa_core::{
    database,
    models::Photo,
    repositories::{
        PgGeoRepository, PgPhotoRepository, face::repository::PgFaceRepository,
        person::repository::PgPersonRepository,
    },
    services::{
        FaceDetectionService, FaceRecognitionService, PhotoEmbedderService, PhotoSearchParams,
        PhotoSearchService,
        embedders::{ClipImageEmbedder, ClipTextEmbedder},
        face_recognition::{RecognitionAction, RecognitionConfig, RecognitionResult},
        photo_scanner,
    },
};
use tabled::{Table, Tabled, settings::Style};

use crate::cli_progress_reporter::CliProgressReporter;

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
    /// Generate image embeddings for photos
    Embed,
    /// Search photos
    Search {
        /// The query string to search for
        #[arg(long = "text", help = "The semantic query string to search for photos")]
        text: Option<String>,

        /// Optional similarity threshold
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

        /// Page number for pagination
        #[arg(
            long = "page",
            help = "Page number for pagination",
            default_value = "1"
        )]
        page: u32,

        /// Optional result limit
        #[arg(
            long = "per_page",
            help = "Maximum number of results per page",
            default_value = "10"
        )]
        per_page: u32,
    },
    /// Face detection and recognition
    #[command(subcommand)]
    Face(FaceCommands),
}

#[derive(Subcommand)]
enum FaceCommands {
    /// Detect and embed faces in photos
    Detect,
    /// Match face with people
    Recognize {
        /// Similarity threshold for clustering faces
        #[arg(
            long = "similarity-threshold",
            help = "Similarity threshold for clustering faces"
        )]
        similarity_threshold: Option<f32>,

        /// Minimum number of faces required to form a cluster
        #[arg(
            long = "min-cluster-size",
            help = "Minimum number of faces required to form a cluster"
        )]
        min_cluster_size: Option<i32>,

        /// Maximum neighbors to consider for each face
        #[arg(
            long = "max-neighbors",
            help = "Maximum neighbors to consider for each face"
        )]
        max_neighbors: Option<i32>,

        /// Auto-assignment threshold - higher confidence required for automatic assignment
        #[arg(
            long = "auto-assign-threshold",
            help = "Auto-assignment threshold for automatic assignment"
        )]
        auto_assign_threshold: Option<f32>,

        /// Minimum faces required to create a new person
        #[arg(
            long = "min-faces-for-new-person",
            help = "Minimum faces required to create a new person"
        )]
        min_faces_for_new_person: Option<i32>,

        /// Preview actions without executing them
        #[arg(long = "dry-run", help = "Preview actions without executing them")]
        dry_run: bool,
    },
}

#[derive(Tabled)]
struct PhotoRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Creation date")]
    pub date_taken: String,
}

impl From<Photo> for PhotoRow {
    fn from(result: Photo) -> Self {
        Self {
            id: result.id,
            path: result.path,
            date_taken: result
                .date_taken_local
                .unwrap_or(result.created_at.naive_local())
                .to_string(),
        }
    }
}

#[derive(Tabled)]
struct RecognitionResultRow {
    #[tabled(rename = "ID")]
    pub cluster_id: i32,
    #[tabled(rename = "Action")]
    pub action: String,
    #[tabled(rename = "Face IDs")]
    pub face_ids: String,
    #[tabled(rename = "Photo paths")]
    pub photo_paths: String,
}

impl From<RecognitionResult> for RecognitionResultRow {
    fn from(result: RecognitionResult) -> Self {
        Self {
            cluster_id: result.cluster_id,
            action: match result.action {
                RecognitionAction::AutoAssignToExisting { person_name, .. } => {
                    format!("✅ Assigned to {}", person_name)
                }
                RecognitionAction::CreateNewPerson { suggested_name, .. } => {
                    format!("🆕 Created {}", suggested_name)
                }
                RecognitionAction::ManualReview { reason, .. } => {
                    format!("⚠️ Review: {:?}", reason)
                }
                RecognitionAction::Reject { reason, .. } => format!("❌ Rejected: {:?}", reason),
            },
            face_ids: format!("{:?}", result.face_ids),
            photo_paths: result.photo_paths.join("\n"),
        }
    }
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn run(self) -> Result<()> {
        let pool = database::create_pool().expect("Failed to create database pool");

        let mut photo_repository = PgPhotoRepository::new(pool.clone());
        let geo_repository = PgGeoRepository::new(pool.clone());
        let person_repository = PgPersonRepository::new(pool.clone());

        match self.command {
            Commands::Scan {
                root_directory,
                with_exif,
                with_hash,
            } => {
                let progress_reporter = CliProgressReporter::new();

                let _ = photo_scanner::scan(
                    &root_directory,
                    &mut photo_repository,
                    with_exif,
                    with_hash,
                    &progress_reporter,
                );

                Ok(())
            }
            Commands::Embed {} => {
                let progress_reporter = CliProgressReporter::new();
                let image_embedder = ClipImageEmbedder::new()?;
                let mut photo_embedder =
                    PhotoEmbedderService::new(photo_repository, image_embedder, progress_reporter);

                photo_embedder.embed()?;

                Ok(())
            }
            Commands::Face(face_command) => match face_command {
                FaceCommands::Detect {} => {
                    let progress_reporter = CliProgressReporter::new();
                    let face_repository = PgFaceRepository::new(pool);
                    let mut face_detection_service = FaceDetectionService::new(
                        photo_repository,
                        face_repository,
                        progress_reporter,
                    );

                    face_detection_service.detect_faces()?;

                    Ok(())
                }
                FaceCommands::Recognize {
                    similarity_threshold,
                    min_cluster_size,
                    max_neighbors,
                    auto_assign_threshold,
                    min_faces_for_new_person,
                    dry_run,
                } => {
                    let config = RecognitionConfig {
                        similarity_threshold: similarity_threshold.unwrap_or(0.6),
                        max_neighbors: max_neighbors.unwrap_or(20),
                        min_cluster_size: min_cluster_size.unwrap_or(3),
                        auto_assign_threshold: auto_assign_threshold.unwrap_or(0.8),
                        min_faces_for_new_person: min_faces_for_new_person.unwrap_or(3),
                    };

                    let face_repository = PgFaceRepository::new(pool.clone());
                    let person_repository = PgPersonRepository::new(pool.clone());
                    let progress_reporter = CliProgressReporter::new();

                    let mut face_recognition_service = FaceRecognitionService::new(
                        face_repository,
                        person_repository,
                        progress_reporter,
                        Some(config),
                    );

                    let result = face_recognition_service.recognize_faces(dry_run)?;

                    let recognition_result_rows: Vec<RecognitionResultRow> =
                        result.results.into_iter().map(|p| p.into()).collect();
                    let mut table = Table::new(recognition_result_rows);
                    table.with(Style::rounded());
                    println!("{}", table);

                    Ok(())
                }
            },
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
                let mut photo_search = PhotoSearchService::new(
                    photo_repository,
                    geo_repository,
                    person_repository,
                    text_embedder,
                );

                let search_params = PhotoSearchParams {
                    text,
                    threshold,
                    country,
                    city,
                    date_from,
                    date_to,
                    page,
                    per_page,
                    ..PhotoSearchParams::default()
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

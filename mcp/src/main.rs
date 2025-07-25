use anyhow::Result;
use rmcp::{
    ErrorData, ServiceExt,
    handler::server::ServerHandler,
    model::{
        Annotated, CallToolRequestMethod, CallToolRequestParam, CallToolResult, Implementation,
        InitializeResult, ListToolsResult, PaginatedRequestParam, ProtocolVersion, RawContent,
        RawTextContent, ServerCapabilities, Tool,
    },
    service::{RequestContext, RoleServer},
    transport::stdio,
};
use serde::{Deserialize, Serialize};
use std::{future::Future, sync::Arc};

use picasa_core::{
    config::Config,
    database,
    services::photo_search::{PhotoSearchService, PhotoSearchParams},
    repositories::{
        PgPhotoRepository, PgGeoRepository, 
        face::repository::PgFaceRepository,
        person::repository::PgPersonRepository,
    },
    services::embedders::ClipTextEmbedder,
};

#[derive(Clone)]
pub struct PicasaServer {
    db_pool: database::DbPool,
    config: Config,
}

impl PicasaServer {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load()
            .map_err(|e| anyhow::Error::msg(format!("Failed to load configuration: {}", e)))?;
        let db_pool = database::create_pool(&config.database)
            .map_err(|e| anyhow::Error::msg(format!("Failed to create database pool: {}", e)))?;
        
        Ok(Self { db_pool, config })
    }
}

impl ServerHandler for PicasaServer {
    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::default(),
            server_info: Implementation {
                name: "picasa-rs".into(),
                version: "0.1.0".into(),
            },
            instructions: Some("A photo search server powered by picasa-rs".into()),
        }
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        async move {
            let input_schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "Semantic text search query (e.g., 'sunset over mountains')"
                    },
                    "threshold": {
                        "type": "number",
                        "minimum": 0.0,
                        "maximum": 1.0,
                        "description": "Similarity threshold for semantic search (0.0-1.0)"
                    },
                    "country": {
                        "type": "string",
                        "description": "Filter by country name"
                    },
                    "city": {
                        "type": "string",
                        "description": "Filter by city name"
                    },
                    "date_from": {
                        "type": "string",
                        "description": "Start date in ISO 8601 format (e.g., '2023-01-01T00:00:00Z')"
                    },
                    "date_to": {
                        "type": "string",
                        "description": "End date in ISO 8601 format (e.g., '2023-12-31T23:59:59Z')"
                    },
                    "page": {
                        "type": "integer",
                        "minimum": 1,
                        "default": 1,
                        "description": "Page number for pagination"
                    },
                    "per_page": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 100,
                        "default": 10,
                        "description": "Number of results per page"
                    }
                }
            });

            let input_schema_map = if let serde_json::Value::Object(obj) = input_schema {
                obj
            } else {
                serde_json::Map::new()
            };

            Ok(ListToolsResult {
                tools: vec![Tool {
                    name: "search_photos".into(),
                    description: Some("Search for photos using text queries, location filters, date ranges, and person filters".into()),
                    input_schema: Arc::new(input_schema_map),
                    annotations: None,
                }],
                next_cursor: None,
            })
        }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        async move {
            match request.name.as_ref() {
                "search_photos" => {
                    let search_params: SearchPhotoParams = serde_json::from_value(
                        serde_json::Value::Object(request.arguments.unwrap_or_default()),
                    )
                    .map_err(|e| {
                        ErrorData::invalid_params(format!("Invalid search parameters: {}", e), None)
                    })?;

                    // Convert our search params to the core PhotoSearchParams format
                    let core_params = PhotoSearchParams {
                        text: search_params.text.clone(),
                        threshold: search_params.threshold,
                        country: search_params.country.clone(),
                        country_id: None,
                        city: search_params.city.clone(),
                        city_id: None,
                        date_from: search_params.date_from.clone(),
                        date_to: search_params.date_to.clone(),
                        person_ids: None,
                        person_match_mode: None,
                        page: search_params.page.unwrap_or(1),
                        per_page: search_params.per_page.unwrap_or(10),
                    };

                    // Set up repositories and search service
                    let photo_repository = PgPhotoRepository::new(self.db_pool.clone());
                    let geo_repository = PgGeoRepository::new(self.db_pool.clone());
                    let person_repository = PgPersonRepository::new(self.db_pool.clone());
                    let face_repository = PgFaceRepository::new(self.db_pool.clone());
                    
                    // Initialize text embedder (this might fail if model files are not available)
                    let text_embedder = match ClipTextEmbedder::new(&self.config.clip_model) {
                        Ok(embedder) => embedder,
                        Err(e) => {
                            // If embedder fails to initialize, we can still search without semantic text search
                            eprintln!("Warning: Failed to initialize text embedder: {}", e);
                            return Ok(CallToolResult {
                                content: vec![Annotated {
                                    raw: RawContent::Text(RawTextContent {
                                        text: format!("Warning: Text embedder not available. Error: {}\n\nFalling back to basic photo search without semantic text queries.", e),
                                    }),
                                    annotations: None,
                                }],
                                is_error: Some(false),
                            });
                        }
                    };

                    // Create and use the search service
                    let mut search_service = PhotoSearchService::new(
                        photo_repository,
                        geo_repository,
                        person_repository,
                        face_repository,
                        text_embedder,
                    );

                    let search_result = match search_service.search(core_params) {
                        Ok(results) => results,
                        Err(e) => {
                            return Err(ErrorData::internal_error(format!("Search failed: {}", e), None));
                        }
                    };

                    // Format results manually since PaginatedResult doesn't implement Serialize
                    let formatted_items: Vec<serde_json::Value> = search_result.items.iter()
                        .map(|photo| serde_json::json!({
                            "id": photo.id,
                            "path": photo.path,
                            "file_name": photo.file_name,
                            "file_size": photo.file_size,
                            "created_at": photo.created_at,
                            "modified_at": photo.modified_at,
                            "hash": photo.hash,
                            "camera_make": photo.camera_make,
                            "camera_model": photo.camera_model,
                            "lens_model": photo.lens_model,
                            "orientation": photo.orientation,
                            "date_taken_local": photo.date_taken_local,
                            "date_taken_utc": photo.date_taken_utc,
                            "image_width": photo.image_width,
                            "image_height": photo.image_height,
                            "face_detection_completed": photo.face_detection_completed,
                            "country_id": photo.country_id,
                            "city_id": photo.city_id,
                            "indexed_at": photo.indexed_at
                        }))
                        .collect();
                    
                    let json_result = serde_json::json!({
                        "items": formatted_items,
                        "total": search_result.total,
                        "page": search_result.page,
                        "per_page": search_result.per_page,
                        "total_pages": search_result.total_pages
                    });
                    
                    Ok(CallToolResult {
                        content: vec![Annotated {
                            raw: RawContent::Text(RawTextContent {
                                text: format!(
                                    "Found {} photos matching your search criteria (page {} of {}):\n{}",
                                    search_result.total,
                                    search_result.page,
                                    search_result.total_pages,
                                    serde_json::to_string_pretty(&json_result).unwrap()
                                ),
                            }),
                            annotations: None,
                        }],
                        is_error: Some(false),
                    })
                }
                _ => Err(ErrorData::method_not_found::<CallToolRequestMethod>()),
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SearchPhotoParams {
    text: Option<String>,
    threshold: Option<f32>,
    country: Option<String>,
    city: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let server = PicasaServer::new()
        .map_err(|e| anyhow::Error::msg(format!("Failed to create server: {}", e)))?;
        
    let service = server.serve(stdio()).await?;

    service.waiting().await?;

    Ok(())
}


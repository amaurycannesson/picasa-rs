# picasar-rs 

A photo scanning tool with AI-powered semantic search capabilities.

## Features

- **Photo Scanning**: Recursively scan directories for photos with optional EXIF data extraction and file hashing
- **AI Embeddings**: Generate CLIP model embeddings for semantic understanding of images
- **Semantic Search**: Search your photo collection using natural language queries
- **Face Detection**: Detect and embed faces in photos using AI
- **Face Recognition**: Automatically cluster and recognize faces, with support for person assignment
- **PostgreSQL Integration**: Uses PostgreSQL with pgvector extension for efficient similarity search
- **HEIC Support**: Native support for Apple's HEIC image format

## Usage

### Scan Photos
```bash
# Basic scan
picasa-cli scan /path/to/photos

# Scan with both EXIF and hashing
picasa-cli scan /path/to/photos --with-exif --with-hash
```

### Generate Embeddings
```bash
# Compute embeddings for all scanned photos
picasa-cli embed
```

### Search Photos
```bash
# Basic semantic search
picasa-cli search --text "sunset over mountains"

# Search with filters
picasa-cli search --text "beach" --country "Spain" --city "Barcelona"

# Search with date range
picasa-cli search --text "vacation" --date-from "2023-01-01T00:00:00Z" --date-to "2023-12-31T23:59:59Z"

# Search with pagination
picasa-cli search --text "mountains" --page 2 --per_page 20

# Search with similarity threshold
picasa-cli search --text "sunset" --threshold 0.8
```

### Face Detection and Recognition
```bash
# Detect faces in photos
picasa-cli face detect

# Recognize and cluster faces
picasa-cli face recognize

# Preview face recognition without changes
picasa-cli face recognize --dry-run

# Recognize with custom parameters
picasa-cli face recognize --similarity-threshold 0.7 --min-cluster-size 5 --auto-assign-threshold 0.9
```

## TODO

- [ ] Improve documentation (install steps)
- [ ] Add integration tests
- [ ] Package CLIP model with binary
- [ ] Add GUI, Web server, MCP server
- [ ] Configuration file system (~/.picasa-rs/config.toml)
- [ ] Create image thumbnails
- [ ] Logging system

## Requirements

- PostgreSQL with pgvector and postgis extensions
- Sys deps : libheif, libpq
- CLIP model files

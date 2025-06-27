# picasar-rs 

A photo scanning tool with AI-powered semantic search capabilities.

## Features

- **Photo Scanning**: Recursively scan directories for photos with optional EXIF data extraction and file hashing
- **AI Embeddings**: Generate CLIP model embeddings for semantic understanding of images
- **Semantic Search**: Search your photo collection using natural language queries
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
picasa-cli search semantic "sunset over mountains"
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

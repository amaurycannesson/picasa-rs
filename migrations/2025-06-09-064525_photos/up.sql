CREATE TABLE photos (
    id SERIAL,

    -- Unique file path
    path TEXT NOT NULL PRIMARY KEY,

    -- File metadata
    file_name TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL,
    indexed_at TIMESTAMP NOT NULL,

    -- File blake3 hash
    hash TEXT,

    -- EXIF metadata
    camera_make TEXT,
    camera_model TEXT,
    lens_model TEXT,
    orientation INTEGER,
    date_taken TIMESTAMP,
    gps_location GEOGRAPHY(POINT, 4326), 
    image_width INTEGER,
    image_height INTEGER,

    -- CLIP embedding
    embedding VECTOR(512)
);

CREATE INDEX IF NOT EXISTS photos_embedding_cosine_idx 
ON photos 
USING ivfflat (embedding vector_cosine_ops) 
WITH (lists = 100);

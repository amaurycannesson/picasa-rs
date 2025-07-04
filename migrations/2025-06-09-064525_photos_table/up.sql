CREATE TABLE IF NOT EXISTS photos (
    id SERIAL PRIMARY KEY,

    -- Unique file path
    path TEXT NOT NULL UNIQUE,

    -- File metadata
    file_name TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    modified_at TIMESTAMPTZ NOT NULL,

    -- File blake3 hash
    hash TEXT,

    -- EXIF metadata
    camera_make TEXT,
    camera_model TEXT,
    lens_model TEXT,
    orientation INTEGER,
    date_taken_local TIMESTAMP,
    date_taken_utc TIMESTAMPTZ,
    gps_location GEOMETRY(POINT, 4326), 
    image_width INTEGER,
    image_height INTEGER,

    -- CLIP embedding
    embedding VECTOR(512),

    -- Face detection
    face_detection_completed BOOLEAN NOT NULL DEFAULT FALSE, 

    -- Foreign keys
    country_id INTEGER REFERENCES countries(gid),
    city_id INTEGER REFERENCES cities(geonameid),

    indexed_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS photos_embedding_cosine_idx ON photos USING hnsw (embedding vector_cosine_ops) WITH (m = 16, ef_construction = 64);
CREATE INDEX IF NOT EXISTS photos_gps_location_gist_idx ON photos USING GIST (gps_location);

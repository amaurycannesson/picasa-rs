CREATE TABLE IF NOT EXISTS faces (
    id SERIAL PRIMARY KEY,

    photo_id INTEGER NOT NULL REFERENCES photos(id) ON DELETE CASCADE,

    -- Face detection
    bbox_x INTEGER NOT NULL,
    bbox_y INTEGER NOT NULL,
    bbox_width INTEGER NOT NULL,
    bbox_height INTEGER NOT NULL,
    confidence REAL NOT NULL,
    gender VARCHAR(10),
    embedding VECTOR(512),
    
    -- Face recognition
    person_id INTEGER REFERENCES people(id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS faces_photo_id_idx ON faces(photo_id);
CREATE INDEX IF NOT EXISTS faces_person_id_idx ON faces(person_id);
CREATE INDEX IF NOT EXISTS faces_embedding_cosine_idx ON faces USING hnsw (embedding vector_cosine_ops) WITH (m = 24, ef_construction = 128);
CREATE OR REPLACE FUNCTION similarity_search(
    query_embedding vector(512),
    similarity_threshold float4 DEFAULT 0.0,
    result_limit integer DEFAULT 10
)
RETURNS TABLE(
    id integer,
    path text,
    similarity float4
) 
LANGUAGE sql STABLE
AS $$
    SELECT 
        photos.id,
        photos.path,
        (1 - (photos.embedding <=> query_embedding))::float4 as similarity
    FROM photos
    WHERE (1 - (photos.embedding <=> query_embedding)) > similarity_threshold
    ORDER BY similarity DESC
    LIMIT result_limit;
$$;

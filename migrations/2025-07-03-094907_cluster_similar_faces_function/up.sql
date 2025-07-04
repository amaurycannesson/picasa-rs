CREATE OR REPLACE FUNCTION cluster_similar_faces(
    similarity_threshold REAL,
    max_neighbors INTEGER,
    min_cluster_size INTEGER
)
RETURNS TABLE(
    cluster_id INTEGER,
    representative_face_id INTEGER,
    face_count INTEGER,
    face_ids INTEGER[],
    photo_paths TEXT[],
    face_ids_without_person INTEGER[],
    face_ids_with_person INTEGER[],
    person_ids INTEGER[],
    avg_similarity_score REAL,
    min_similarity_score REAL 
) AS $$
BEGIN
    RETURN QUERY
    WITH RECURSIVE face_similarities AS (
        SELECT 
            f1.id as face1_id,
            f2.id as face2_id
        FROM faces f1
        JOIN LATERAL (
            SELECT id, embedding
            FROM faces f2
            WHERE f2.id > f1.id
            ORDER BY f1.embedding <=> f2.embedding
            LIMIT max_neighbors
        ) f2 ON (1 - (f1.embedding <=> f2.embedding)) >= similarity_threshold
    ),

    -- Union-Find: each face points to parent (initially itself)
    union_find AS (
        -- Base case: each face is its own parent
        SELECT DISTINCT face1_id as face_id, face1_id as parent_id FROM face_similarities
        UNION 
        SELECT DISTINCT face2_id as face_id, face2_id as parent_id FROM face_similarities
        
        UNION
        
        -- Recursive case: union operation - point to smaller parent
        SELECT 
            uf.face_id,
            LEAST(uf.parent_id, 
                CASE WHEN fs.face1_id = uf.face_id THEN fs.face2_id ELSE fs.face1_id END) as parent_id
        FROM union_find uf
        JOIN face_similarities fs ON (fs.face1_id = uf.face_id OR fs.face2_id = uf.face_id)
        WHERE LEAST(uf.parent_id, 
                    CASE WHEN fs.face1_id = uf.face_id THEN fs.face2_id ELSE fs.face1_id END) < uf.parent_id
    ),

    -- Find root parent for each face (path compression)
    final_parents AS (
        SELECT 
            face_id,
            MIN(parent_id) as root_parent
        FROM union_find
        GROUP BY face_id
    ),

    similarity_stats AS (
        SELECT 
            fp.root_parent,
            AVG(1.0 - (f1.embedding <=> f2.embedding))::REAL as avg_sim,
            MIN(1.0 - (f1.embedding <=> f2.embedding))::REAL as min_sim
        FROM final_parents fp
        JOIN faces f1 ON f1.id = fp.face_id
        JOIN faces f2 ON f2.id = fp.root_parent
        GROUP BY fp.root_parent
    )

    SELECT 
        ROW_NUMBER() OVER (ORDER BY COUNT(*) DESC)::INTEGER as cluster_id,
        fp.root_parent as representative_face_id,
        COUNT(*)::INTEGER as face_count,
        ARRAY_AGG(fp.face_id ORDER BY fp.face_id) as face_ids,
        ARRAY_AGG(DISTINCT p.path ORDER BY p.path) as photo_paths,
        COALESCE(ARRAY_AGG(fp.face_id ORDER BY fp.face_id) FILTER (WHERE f.person_id IS NULL), '{}') as face_ids_without_person,
        COALESCE(ARRAY_AGG(fp.face_id ORDER BY fp.face_id) FILTER (WHERE f.person_id IS NOT NULL), '{}') as face_ids_with_person,
        COALESCE(ARRAY_AGG(DISTINCT f.person_id ORDER BY f.person_id) FILTER (WHERE f.person_id IS NOT NULL), '{}') as person_ids,
        COALESCE(ss.avg_sim, 0::REAL) as avg_similarity_score,
        COALESCE(ss.min_sim, 0::REAL) as min_similarity_score
    FROM final_parents fp
    LEFT JOIN faces f ON f.id = fp.face_id
    LEFT JOIN photos p ON p.id = f.photo_id
    LEFT JOIN similarity_stats ss ON ss.root_parent = fp.root_parent
    GROUP BY fp.root_parent, ss.avg_sim, ss.min_sim
    HAVING COUNT(*) >= min_cluster_size
    AND COUNT(*) FILTER (WHERE f.person_id IS NULL) > 0
    ORDER BY face_count DESC;
END;
$$ LANGUAGE plpgsql;

SELECT AddGeometryColumn('cities', 'geom', 4326, 'POINT', 2);
UPDATE cities SET geom = ST_SetSRID(ST_MakePoint(longitude, latitude), 4326);

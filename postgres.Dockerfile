FROM postgis/postgis:17-3.5

ARG PG_MAJOR=17
ARG POSTGRES_USER=postgres

# Install pgvector extension
RUN apt-get update && \
		apt-mark hold locales && \
		apt-get install -y --no-install-recommends git build-essential postgresql-server-dev-$PG_MAJOR
RUN git clone https://github.com/pgvector/pgvector.git /tmp/pgvector && \
        cd /tmp/pgvector && \
        git checkout v0.8.0
RUN cd /tmp/pgvector && \
		make clean && \
		make OPTFLAGS="" && \
		make install && \
		mkdir /usr/share/doc/pgvector && \
		cp LICENSE README.md /usr/share/doc/pgvector && \
		rm -r /tmp/pgvector && \
		apt-get remove -y git build-essential postgresql-server-dev-$PG_MAJOR

# Install shp2pgsql and download countries data
RUN apt-get install -y --no-install-recommends postgis unzip wget
RUN cd /tmp && \
		wget https://naciscdn.org/naturalearth/packages/natural_earth_vector.zip && \
		unzip natural_earth_vector.zip -d natural_earth_vector && \
		rm natural_earth_vector.zip && \
		chown -R $POSTGRES_USER:$POSTGRES_USER natural_earth_vector && \
		chmod -R u+w natural_earth_vector

RUN apt-get autoremove -y && \
		apt-mark unhold locales && \
		rm -rf /var/lib/apt/lists/*

# Download cities data
RUN cd /tmp && \
		wget https://download.geonames.org/export/dump/cities5000.zip && \
		unzip cities5000.zip -d cities5000 && \
		rm cities5000.zip && \
		chown -R $POSTGRES_USER:$POSTGRES_USER cities5000 && \
		chmod -R u+w cities5000

# Load countries data after PostGIS extension init script
RUN shp2pgsql -I -s 4326 /tmp/natural_earth_vector/10m_cultural/ne_10m_admin_0_countries.shp countries > /docker-entrypoint-initdb.d/11_init_countries_table.sql

# Load cities data
COPY migrations/2025-06-08-080652_cities_table/up.sql /docker-entrypoint-initdb.d/12_create_cities_table.sql
RUN cat << 'EOF' > /docker-entrypoint-initdb.d/13_load_cities_data.sql
COPY cities 
FROM '/tmp/cities5000/cities5000.txt'
WITH (
  FORMAT csv,
  DELIMITER '	',
  NULL ''
);
EOF

# Remove data
RUN echo '#!/usr/bin/env bash' > /docker-entrypoint-initdb.d/14_clean_data.sh && \
		echo 'set -e' >> /docker-entrypoint-initdb.d/14_clean_data.sh && \
		echo 'rm -rf /tmp/*' >> /docker-entrypoint-initdb.d/14_clean_data.sh && \
		chmod +x /docker-entrypoint-initdb.d/14_clean_data.sh

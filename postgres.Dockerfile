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
		apt-get remove -y build-essential postgresql-server-dev-$PG_MAJOR

# Install shp2pgsql and Natural Earth data
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

# Load Natural Earth data after PostGIS extension init script
RUN shp2pgsql -I -s 4326 /tmp/natural_earth_vector/10m_cultural/ne_10m_admin_0_countries.shp countries >> /docker-entrypoint-initdb.d/11_init_countries_table.sql && \
		echo '#!/usr/bin/env bash' > /docker-entrypoint-initdb.d/12_clean_natural_earth_data.sh && \
		echo 'set -e' >> /docker-entrypoint-initdb.d/12_clean_natural_earth_data.sh && \
		echo 'cd /tmp' >> /docker-entrypoint-initdb.d/12_clean_natural_earth_data.sh && \
		echo 'rm -rf natural_earth_vector' >> /docker-entrypoint-initdb.d/12_clean_natural_earth_data.sh && \
		chmod +x /docker-entrypoint-initdb.d/12_clean_natural_earth_data.sh

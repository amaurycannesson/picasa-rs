use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use nom_exif::{Exif, ExifIter, GPSInfo, MediaParser, MediaSource};
use postgis_diesel::types::Point;

pub mod progress_reporter;

/// Convert EXIF GPSInfo to PostGIS Point
pub fn convert_exif_gps_info_to_postgis_point(gps_info: GPSInfo) -> Option<Point> {
    let lat_decimal = convert_gps_coordinate(
        gps_info.latitude.0.into(),
        gps_info.latitude.1.into(),
        gps_info.latitude.2.into(),
    );

    let lon_decimal = convert_gps_coordinate(
        gps_info.longitude.0.into(),
        gps_info.longitude.1.into(),
        gps_info.longitude.2.into(),
    );

    let lat_final = if gps_info.latitude_ref == 'S' {
        -lat_decimal
    } else {
        lat_decimal
    };
    let lon_final = if gps_info.longitude_ref == 'W' {
        -lon_decimal
    } else {
        lon_decimal
    };

    Some(Point::new(lon_final, lat_final, Some(4326)))
}

/// Convert GPS coordinates from degrees/minutes/seconds to decimal degrees
pub fn convert_gps_coordinate(
    degrees: (u32, u32),
    minutes: (u32, u32),
    seconds: (u32, u32),
) -> f64 {
    let deg = degrees.0 as f64 / degrees.1 as f64;
    let min = minutes.0 as f64 / minutes.1 as f64;
    let sec = seconds.0 as f64 / seconds.1 as f64;
    deg + (min / 60.0) + (sec / 3600.0)
}

/// Efficiently compute BLAKE3 hash of a file
pub fn compute_file_hash<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut hasher = blake3::Hasher::new();

    let mut buffer = [0; 64 * 1024];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

/// Extract EXIF data
pub fn extract_exif<P: AsRef<Path>>(path: P) -> Option<Exif> {
    let media_source = MediaSource::file_path(path).ok()?;
    if !media_source.has_exif() {
        return None;
    }
    let iter: ExifIter = MediaParser::new().parse(media_source).ok()?;
    Some(iter.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom_exif::{GPSInfo, LatLng};

    #[test]
    fn test_should_convert_gps_coordinates_from_degrees_minutes_seconds_to_decimal() {
        // Test conversion of 40°26'46"N (40.446111°)
        let degrees = (40, 1); // 40/1 = 40 degrees
        let minutes = (26, 1); // 26/1 = 26 minutes
        let seconds = (46, 1); // 46/1 = 46 seconds

        let decimal = convert_gps_coordinate(degrees, minutes, seconds);

        // 40 + (26/60) + (46/3600) = 40 + 0.433333 + 0.012778 = 40.446111
        assert!((decimal - 40.446111).abs() < 0.000001);
    }

    #[test]
    fn test_should_convert_exif_gps_info_to_postgis_point() {
        let gps_info = GPSInfo {
            latitude: LatLng::from([
                (40, 1), // 40 degrees
                (26, 1), // 26 minutes
                (46, 1), // 46 seconds
            ]),
            latitude_ref: 'N',
            longitude: LatLng::from([
                (79, 1), // 79 degrees
                (58, 1), // 58 minutes
                (56, 1), // 56 seconds
            ]),
            longitude_ref: 'W',
            altitude: LatLng::from([(0, 0), (0, 0), (0, 0)]).0,
            altitude_ref: 0,
            speed: None,
            speed_ref: None,
        };

        let point = convert_exif_gps_info_to_postgis_point(gps_info).unwrap();

        assert_eq!(point.x, -79.98222222222222);
        assert_eq!(point.y, 40.44611111111111);
        assert_eq!(point.srid, Some(4326));
    }
}

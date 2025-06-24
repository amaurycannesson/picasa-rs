use std::time::SystemTime;

use chrono::{DateTime, Local, NaiveDateTime};
use nom_exif::GPSInfo;
use postgis_diesel::types::Point;

pub mod progress_reporter;

/// Convert a SystemTime to NaiveDateTime
pub fn system_time_to_naive_datetime(time: SystemTime) -> NaiveDateTime {
    DateTime::<Local>::from(time).naive_utc()
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use nom_exif::{GPSInfo, LatLng};
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_should_convert_system_time_to_naive_datetime() {
        let seconds_since_epoch = 1704110400; // 2024-01-01 12:00:00 UTC
        let system_time = UNIX_EPOCH + Duration::from_secs(seconds_since_epoch);

        let result = system_time_to_naive_datetime(system_time);

        assert_eq!(result.and_utc().to_rfc3339(), "2024-01-01T12:00:00+00:00");
    }

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

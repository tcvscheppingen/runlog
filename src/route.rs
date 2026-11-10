use gpx::Gpx;
use gpx::Track;
use gpx::read;
use std::{fs::File, io::BufReader};

const EARTH_RADIUS: f64 = 6371008.7714;

/// Use the Haversine formula to calculate the distance between two coordinates.
fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let a: f64 = ((lat2 - lat1) / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * ((lon2 - lon1) / 2.0).sin().powi(2);
    // 2.0 * EARTH_RADIUS * a.sqrt().atan2((1.0 - a).sqrt())
    2.0 * EARTH_RADIUS * a.sqrt().asin()
}

/// Load a GPX file from path.
pub fn load_gpx(path: &str) -> Result<Gpx, String> {
    let file: File = File::open(path).map_err(|e| format!("Unable to open {path}: {e}"))?;

    let reader: BufReader<File> = BufReader::new(file);

    read(reader).map_err(|e| format!("Unable to parse gpx in {path}: {e}"))
}

/// Calculate the distance of a route in a GPX file.
pub fn route_distance(track: &Track) -> Result<f64, String> {
    let mut total_distance: f64 = 0.0;

    if track.segments.is_empty() {
        return Err("Track contains no segments".to_string());
    }

    for segment in &track.segments {
        if segment.points.is_empty() {
            return Err("Segment contains no points".to_string());
        }

        for pair in segment.points.windows(2) {
            let (p1, p2) = (pair[0].point(), pair[1].point());

            total_distance += haversine(
                p1.y().to_radians(),
                p1.x().to_radians(),
                p2.y().to_radians(),
                p2.x().to_radians(),
            );
        }
    }

    Ok(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::Point;
    use gpx::{TrackSegment, Waypoint};

    // Build a segment from (longitude, latitude) pairs in degrees
    fn segment(points: &[(f64, f64)]) -> TrackSegment {
        let mut segment: TrackSegment = TrackSegment::new();
        for &(lon, lat) in points {
            segment.points.push(Waypoint::new(Point::new(lon, lat)));
        }
        segment
    }

    // Distance of one degree along a meridian
    fn one_degree() -> f64 {
        EARTH_RADIUS * 1.0_f64.to_radians()
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-6,
            "expected {expected}, got {actual}"
        );
    }

    // --- haversine ---

    #[test]
    fn haversine_same_point_is_zero() {
        let lat: f64 = 52.0_f64.to_radians();
        let lon: f64 = 5.0_f64.to_radians();
        assert_eq!(haversine(lat, lon, lat, lon), 0.0);
    }

    #[test]
    fn haversine_one_degree_along_meridian() {
        // Along a meridian, the distance is simply the radius times the angle
        assert_close(haversine(0.0, 0.0, 1.0_f64.to_radians(), 0.0), one_degree());
    }

    #[test]
    fn haversine_is_symmetric() {
        let (lat1, lon1) = (52.09_f64.to_radians(), 5.12_f64.to_radians());
        let (lat2, lon2) = (52.37_f64.to_radians(), 4.90_f64.to_radians());
        assert_eq!(
            haversine(lat1, lon1, lat2, lon2),
            haversine(lat2, lon2, lat1, lon1)
        );
    }

    // --- route_distance ---

    #[test]
    fn route_distance_sums_points_in_segment() {
        let mut track: Track = Track::new();
        track.segments.push(segment(&[(0.0, 0.0), (0.0, 1.0), (0.0, 2.0)]));

        assert_close(route_distance(&track).unwrap(), 2.0 * one_degree());
    }

    #[test]
    fn route_distance_sums_segments_without_gap_between_them() {
        // The jump from longitude 0 to 10 between segments (e.g. a paused
        // watch) must not count towards the distance
        let mut track: Track = Track::new();
        track.segments.push(segment(&[(0.0, 0.0), (0.0, 1.0)]));
        track.segments.push(segment(&[(10.0, 0.0), (10.0, 1.0)]));

        assert_close(route_distance(&track).unwrap(), 2.0 * one_degree());
    }

    #[test]
    fn route_distance_rejects_track_without_segments() {
        let track: Track = Track::new();
        assert_eq!(
            route_distance(&track),
            Err("Track contains no segments".to_string())
        );
    }

    #[test]
    fn route_distance_rejects_segment_without_points() {
        let mut track: Track = Track::new();
        track.segments.push(TrackSegment::new());
        assert_eq!(
            route_distance(&track),
            Err("Segment contains no points".to_string())
        );
    }
}

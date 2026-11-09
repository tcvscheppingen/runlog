use std::{fs::File, io::BufReader};

use gpx::Gpx;
use gpx::Track;
use gpx::TrackSegment;
use gpx::read;

const EARTH_RADIUS: f64 = 6371008.7714;

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let a: f64 = ((lat2 - lat1) / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * ((lon2 - lon1) / 2.0).sin().powi(2);
    // 2.0 * EARTH_RADIUS * a.sqrt().atan2((1.0 - a).sqrt())
    2.0 * EARTH_RADIUS * a.sqrt().asin()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename: String = std::env::args().nth(1).ok_or("Usage runlog <file.gpx")?;

    let file: File =
        File::open(&filename).map_err(|e| format!("Unable to open {filename}: {e}"))?;
    let reader: BufReader<File> = BufReader::new(file);

    let gpx: Gpx = match read(reader) {
        Ok(gpx) => gpx,
        Err(e) => {
            eprintln!("Unable to parse gpx in {filename}: {e}");
            std::process::exit(1);
        }
    };

    let track: &Track = gpx.tracks.first().ok_or("GPX file contains no tracks")?;
    let segment: &TrackSegment = track.segments.first().ok_or("Track contains no segments")?;

    let mut total_distance: f64 = 0.0;

    for pair in segment.points.windows(2) {
        let (p1, p2) = (pair[0].point(), pair[1].point());

        total_distance += haversine(
            p1.y().to_radians(),
            p1.x().to_radians(),
            p2.y().to_radians(),
            p2.x().to_radians(),
        );
    }

    println!("Total distance: {:.2} km", total_distance / 1000.0);

    Ok(())
}

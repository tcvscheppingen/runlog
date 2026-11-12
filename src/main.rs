//! Keep track of your runs with this simple CLI tool.
//!

use gpx::Gpx;
use gpx::Track;

mod duration;
use duration::parse_duration;

mod pace;
use pace::calculate_pace;
use pace::format_pace;

mod route;
use route::load_gpx;
use route::route_distance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename: String = std::env::args()
        .nth(1)
        .ok_or("Usage: runlog <file.gpx> <duration>")?;
    let duration: String = std::env::args()
        .nth(2)
        .ok_or("Provide a duration in minutes")?;

    let gpx: Gpx = load_gpx(&filename)?;

    let track: &Track = gpx.tracks.first().ok_or("GPX file contains no tracks")?;

    let total_distance: f64 = route_distance(track)?;

    println!("Total distance: {:.2} km", total_distance / 1000.0);

    // let duration: f64 =
    //     parse_duration(&duration).map_err(|e| format!("Invalid duration {duration}: {e}"))?;
    // let pace: f64 = calculate_pace(total_distance, duration);
    //
    let time_input: Vec<_> = duration.split(":").collect();

    println!("{:?}", time_input);

    for split in time_input {
        println!("{split}");
    }

    // println!("Pace: {} min/km", format_pace(pace));

    Ok(())
}

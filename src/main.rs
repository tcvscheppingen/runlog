//! Keep track of your runs with this simple CLI tool.
//!

use clap::Arg;
use clap::command;

mod duration;
use duration::parse_duration;

mod pace;
use pace::calculate_pace;
use pace::format_pace;

mod route;

use crate::route::get_distance_from_route;
use crate::route::print_distance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!("runlog")
        .help_template(
            "\
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
     ",
        )
        .author("Thymen van Scheppingen")
        .about("A minimalist running tracker")
        .arg_required_else_help(true)
        .arg(
            Arg::new("route")
                .short('r')
                .long("route")
                .help("A GPX file containing a route"),
        )
        .arg(
            Arg::new("duration")
                .short('d')
                .long("duration")
                .help("Use with -r to get the average pace of the run"),
        )
        .get_matches();

    let route: Option<&String> = matches.get_one::<String>("route");
    let duration: Option<&String> = matches.get_one::<String>("duration");

    if let Some(route) = route
        && let Some(duration) = duration
    {
        let distance: f64 = get_distance_from_route(route)?;
        print_distance(distance);

        let duration: f64 =
            parse_duration(duration).map_err(|e| format!("Invalid duration {duration}: {e}"))?;
        let pace: f64 = calculate_pace(distance, duration);

        println!("Pace: {} min/km", format_pace(pace));
    } else if let Some(route) = route {
        let distance: f64 = get_distance_from_route(route)?;
        print_distance(distance);
    }

    Ok(())
}

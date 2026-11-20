//! Keep track of your runs with this simple CLI tool.
//!

use std::path::PathBuf;

use clap::Arg;
use clap::ArgGroup;
use clap::ArgMatches;
use clap::Command;
use clap::command;
use clap::value_parser;

mod distance;

mod duration;

mod pace;
use pace::calculate_pace;
use pace::format_pace;

mod route;

use crate::distance::kilometers_to_meters;
use crate::route::get_distance_from_route;
use crate::route::get_formatted_distance;

mod run_log_file;

use crate::run_log_file::Record;
use crate::run_log_file::add_record;
use crate::run_log_file::create_run_log_file;

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
        .subcommand_required(true)
        .subcommand(
            Command::new("dist")
                .arg_required_else_help(true)
                .alias("distance")
                .about("calculate the distance of a route in a GPX file")
                .arg(
                    Arg::new("route")
                        .value_parser(value_parser!(PathBuf))
                        .help("Get the distance of a route from a GPX file"),
                ),
        )
        .subcommand(
            generate_pace_args(Command::new("pace"), false)
                .about("use a route and duration to calculate the distance and pace of a run"),
        )
        .subcommand(
            Command::new("init")
                .about("initialize a running log CSV file")
                .arg(
                    Arg::new("filename")
                        .default_value("running-log.csv")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            generate_pace_args(Command::new("add"), true)
                .about("add a new record to the running log")
                .arg(
                    Arg::new("filename")
                        .default_value("running-log.csv")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let filename = init_matches
                .get_one::<PathBuf>("filename")
                .expect("filename has a default value");

            create_run_log_file(filename)?;
            println!("Created {}", filename.display());
        }
        Some(("pace", pace_matches)) => {
            let duration = pace_matches.get_one::<f64>("duration").copied();

            let distance: f64 = get_distance(pace_matches)?;
            println!("Total distance: {}", get_formatted_distance(distance));

            if let Some(duration) = duration {
                let pace: f64 = calculate_pace(distance, duration);

                println!("Pace: {}", format_pace(pace));
            }
        }
        Some(("add", add_matches)) => {
            let duration = add_matches.get_one::<f64>("duration").copied();

            let distance: f64 = get_distance(add_matches)?;

            let current_date = jiff::ZonedDateTime::now().strftime("%d-%m-%Y");

            let path = add_matches
                .get_one::<PathBuf>("filename")
                .expect("expected filename");

            let formatted_distance = get_formatted_distance(distance);

            if let Some(duration) = duration {
                let pace: f64 = calculate_pace(distance, duration);
                let pace: String = format_pace(pace);
                let record: Record = Record {
                    date: &current_date.to_string(),
                    distance: &formatted_distance,
                    duration, // TODO: use HH:MM:SS format.
                    pace: &pace,
                };

                println!(
                    "Added record: {}, {}, {} to '{}'",
                    formatted_distance,
                    duration,
                    pace,
                    path.display()
                );
                add_record(path, record)?
            }
        }
        Some(("dist", distance_matches)) => {
            let route = distance_matches.get_one::<PathBuf>("route");

            if let Some(route) = route {
                let distance: f64 = get_distance_from_route(route)?;
                println!("Total distance: {}", get_formatted_distance(distance));
            }
        }
        // subcommand_required and the known subcommands are enforced by clap
        _ => unreachable!("clap rejects a missing or unknown subcommand"),
    }

    Ok(())
}

/// Get the distance of a run in metres, from either a GPX route or a distance in kilometres.
fn get_distance(matches: &ArgMatches) -> Result<f64, String> {
    let route = matches.get_one::<PathBuf>("route");
    let distance = matches.get_one::<f64>("distance").copied();

    match (route, distance) {
        (Some(route), None) => get_distance_from_route(route),

        (None, Some(distance)) => Ok(kilometers_to_meters(distance)),
        // The required "length" group allows exactly one of --route and --distance
        _ => unreachable!("clap requires either --route or --distance"),
    }
}

fn generate_pace_args(cmd: Command, duration_required: bool) -> Command {
    cmd.arg(
        Arg::new("route")
            .value_parser(value_parser!(PathBuf))
            .short('r')
            .long("route")
            .help("Get the distance of a route from a GPX file"),
    )
    .arg(
        Arg::new("distance")
            .long("distance")
            .value_parser(distance::parse_distance)
            .help("Distance of the run in kilometres"),
    )
    .arg(
        Arg::new("duration")
            .value_parser(duration::parse_duration)
            .short('d')
            .long("duration")
            .required(duration_required)
            .help("Use with --route or --distance to get the average pace of the run"),
    )
    .group(
        ArgGroup::new("length")
            .args(["route", "distance"])
            .required(true),
    )
    .arg_required_else_help(true)
}

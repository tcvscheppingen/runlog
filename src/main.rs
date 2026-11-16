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

use crate::route::get_distance_from_route;
use crate::route::print_distance;

mod run_log_file;

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
        .arg_required_else_help(true)
        .subcommand(
            Command::new("pace")
                .about("use a route and duration to calculate the distance and pace of a run")
                .arg(
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
                        .help("Use with --route or --distance to get the average pace of the run"),
                )
                .group(
                    ArgGroup::new("length")
                        .args(["route", "distance"])
                        .required(true),
                ),
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
            print_distance(distance);

            if let Some(duration) = duration {
                let pace: f64 = calculate_pace(distance, duration);

                println!("Pace: {} min/km", format_pace(pace));
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
        (None, Some(distance)) => Ok(distance * 1000.0),
        // The required "length" group allows exactly one of --route and --distance
        _ => unreachable!("clap requires either --route or --distance"),
    }
}

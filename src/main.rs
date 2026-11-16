//! Keep track of your runs with this simple CLI tool.
//!

use std::path::PathBuf;

use clap::Arg;
use clap::Command;
use clap::command;
use clap::value_parser;

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
        .arg_required_else_help(true)
        .args_conflicts_with_subcommands(true)
        .arg(
            Arg::new("route")
                .value_parser(value_parser!(PathBuf))
                .short('r')
                .long("route")
                .help("Get the distance of a route from a GPX file"),
        )
        .arg(
            Arg::new("duration")
                .requires("route")
                .value_parser(duration::parse_duration)
                .short('d')
                .long("duration")
                .help("Use with -r to get the average pace of the run"),
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
        Some((name, _)) => unreachable!("clap rejects unknown subcommand {name}"),
        None => {
            let route = matches.get_one::<PathBuf>("route");
            let duration = matches.get_one::<f64>("duration").copied();

            if let Some(route) = route {
                let distance: f64 = get_distance_from_route(route)?;
                print_distance(distance);

                if let Some(duration) = duration {
                    let pace: f64 = calculate_pace(distance, duration);

                    println!("Pace: {} min/km", format_pace(pace));
                }
            }
        }
    }

    Ok(())
}

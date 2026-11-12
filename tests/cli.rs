use std::path::PathBuf;
use std::process::{Command, Output};

use rstest::rstest;

// Path to a file in tests/fixtures
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

// Run the compiled binary with the given arguments
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_runlog"))
        .args(args)
        .output()
        .expect("Failed to run runlog")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[rstest]
#[case::whole_minutes("18")]
#[case::minutes_and_seconds("18:00")]
#[case::hours_minutes_and_seconds("0:18:00")]
fn valid_route_prints_distance_and_pace(#[case] duration: &str) {
    // Every case is the same 18 minute run, so the pace must match
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let output = run(&[path.to_str().unwrap(), duration]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stdout(&output).trim(),
        "Total distance: 3.16 km\nPace: 5:42 min/km"
    );
}

#[test]
fn missing_argument_prints_usage() {
    let output = run(&[]);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("Usage: runlog <file.gpx>"));
}

#[rstest]
#[case::non_numeric("abc")]
#[case::minutes_of_sixty_with_hours("1:60:00")]
fn invalid_duration_fails_without_panic(#[case] duration: &str) {
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let output = run(&[path.to_str().unwrap(), duration]);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains(&format!("Invalid duration {duration}")));
}

#[rstest]
#[case::missing_file("does-not-exist.gpx", "Unable to open")]
#[case::not_gpx("not-gpx.txt", "Unable to parse gpx in")]
#[case::no_tracks("no-tracks.gpx", "GPX file contains no tracks")]
#[case::no_segments("no-segments.gpx", "Track contains no segments")]
fn broken_gpx_file_fails_without_panic(#[case] name: &str, #[case] expected: &str) {
    let path = fixture(name);
    let output = run(&[path.to_str().unwrap(), "18"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains(expected),
        "got: {}",
        stderr(&output)
    );
}

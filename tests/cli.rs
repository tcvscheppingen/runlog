use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use rstest::rstest;

const LOG_HEADER: &str = "Date,Distance,Duration,Pace\n";

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Create an empty directory for a single test, removing leftovers from earlier runs.
fn scratch_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("Failed to create scratch dir");
    dir
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_runlog"))
        .args(args)
        .output()
        .expect("Failed to run runlog")
}

fn run_in(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_runlog"))
        .current_dir(dir)
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
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let path = path.to_str().unwrap();

    let output = run(&["pace", "--route", path, "--duration", duration]);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stdout(&output).trim(),
        "Total distance: 3.16 km\nPace: 5:42 min/km"
    );
}

#[test]
fn distance_subcomand_prints_distance() {
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let path = path.to_str().unwrap();

    let output = run(&["dist", path]);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output).trim(), "Total distance: 3.16 km");
}

#[test]
fn no_subcommand_prints_clap_help() {
    let output = run(&[]);

    assert_eq!(output.status.code(), Some(2));

    let help = format!("{}{}", stdout(&output), stderr(&output));
    assert!(help.contains("Usage:"));
    assert!(help.contains("pace"));
    assert!(help.contains("init"));
    assert!(help.contains("dist"));
    assert!(help.contains("add"));
}

#[test]
fn route_without_duration_prints_distance() {
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let path = path.to_str().unwrap();

    let output = run(&["pace", "-r", path]);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output).trim(), "Total distance: 3.16 km");
}

#[test]
fn duration_without_route_fails() {
    let output = run(&["pace", "--duration", "18:00"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(
        stderr(&output).contains("--route"),
        "got: {}",
        stderr(&output)
    );
}

#[rstest]
#[case::non_numeric("abc", "Invalid minutes 'abc'")]
#[case::minutes_of_sixty_with_hours("1:60:00", "Minutes must be less than 60")]
fn invalid_duration_fails_without_panic(#[case] duration: &str, #[case] reason: &str) {
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let path = path.to_str().unwrap();

    let output = run(&["pace", "--route", path, "--duration", duration]);

    assert_eq!(output.status.code(), Some(2));

    let stderr = stderr(&output);
    assert!(
        stderr.contains(&format!("invalid value '{duration}' for '--duration")),
        "got: {stderr}"
    );
    assert!(stderr.contains(reason), "got: {stderr}");
}

#[rstest]
#[case::missing_file("does-not-exist.gpx", "Unable to open")]
#[case::not_gpx("not-gpx.txt", "Unable to parse gpx in")]
#[case::no_tracks("no-tracks.gpx", "GPX file contains no tracks")]
#[case::no_segments("no-segments.gpx", "Track contains no segments")]
fn broken_gpx_file_fails_without_panic(#[case] name: &str, #[case] expected: &str) {
    let path = fixture(name);
    let path = path.to_str().unwrap();

    let output = run(&["pace", "--route", path]);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains(expected),
        "got: {}",
        stderr(&output)
    );
}

#[test]
fn init_without_filename_creates_default_log() {
    let dir = scratch_dir("init-default");

    let output = run_in(&dir, &["init"]);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let contents = fs::read_to_string(dir.join("running-log.csv")).expect("Log file not created");
    assert_eq!(contents, LOG_HEADER);
}

#[test]
fn init_with_filename_creates_that_file() {
    let dir = scratch_dir("init-custom");

    let output = run_in(&dir, &["init", "my-runs.csv"]);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let contents = fs::read_to_string(dir.join("my-runs.csv")).expect("Log file not created");
    assert_eq!(contents, LOG_HEADER);
    assert!(!dir.join("running-log.csv").exists());
}

#[test]
fn init_refuses_to_overwrite_existing_log() {
    let dir = scratch_dir("init-existing");
    let log = dir.join("running-log.csv");
    let existing = "Date,Distance,Pace\n2026-11-16,3160,5:42\n";
    fs::write(&log, existing).unwrap();

    let output = run_in(&dir, &["init"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("running-log.csv"),
        "got: {}",
        stderr(&output)
    );
    assert_eq!(fs::read_to_string(&log).unwrap(), existing);
}

#[test]
fn init_into_missing_directory_fails_without_panic() {
    let dir = scratch_dir("init-missing-dir");

    let output = run_in(&dir, &["init", "does-not-exist/running-log.csv"]);

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn init_cannot_be_combined_with_route() {
    let dir = scratch_dir("init-with-route");
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let path = path.to_str().unwrap();

    let output = run_in(&dir, &["--route", path, "init"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(!dir.join("running-log.csv").exists());
}

#[rstest]
#[case::matches_fixture_route("3.16", "18", "Total distance: 3.16 km\nPace: 5:42 min/km")]
#[case::whole_kilometres("10", "50:00", "Total distance: 10.00 km\nPace: 5:00 min/km")]
#[case::with_hours("21.1", "1:45:30", "Total distance: 21.10 km\nPace: 5:00 min/km")]
fn distance_with_duration_prints_pace(
    #[case] distance: &str,
    #[case] duration: &str,
    #[case] expected: &str,
) {
    let output = run(&["pace", "--distance", distance, "--duration", duration]);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output).trim(), expected);
}

#[test]
fn distance_without_duration_prints_distance() {
    let output = run(&["pace", "--distance", "5"]);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output).trim(), "Total distance: 5.00 km");
}

#[test]
fn route_and_distance_cannot_be_combined() {
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let path = path.to_str().unwrap();

    let output = run(&["pace", "--route", path, "--distance", "5"]);

    assert_eq!(output.status.code(), Some(2));
    let stderr = stderr(&output);
    assert!(stderr.contains("cannot be used with"), "got: {stderr}");
    assert!(stderr.contains("--route"), "got: {stderr}");
    assert!(stderr.contains("--distance"), "got: {stderr}");
}

#[test]
fn pace_without_route_or_distance_fails() {
    let output = run(&["pace"]);

    assert_eq!(output.status.code(), Some(2));
    let stderr = stderr(&output);
    assert!(stderr.contains("--route"), "got: {stderr}");
    assert!(stderr.contains("--distance"), "got: {stderr}");
}

#[rstest]
#[case::non_numeric("abc")]
#[case::zero("0")]
#[case::negative("-5")]
#[case::not_a_number("NaN")]
#[case::infinite("inf")]
fn invalid_distance_fails_without_panic(#[case] distance: &str) {
    // `--distance=<value>` so clap doesn't read "-5" as a flag
    let output = run(&["pace", &format!("--distance={distance}")]);

    assert_eq!(output.status.code(), Some(2));
    let stderr = stderr(&output);
    assert!(
        stderr.contains(&format!("invalid value '{distance}' for '--distance")),
        "got: {stderr}"
    );
}

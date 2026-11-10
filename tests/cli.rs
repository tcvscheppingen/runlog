use std::path::PathBuf;
use std::process::{Command, Output};

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

#[test]
fn valid_route_prints_total_distance() {
    let path = fixture("onthegomap-3.1-km-route.gpx");
    let output = run(&[path.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output).trim(), "Total distance: 3.16 km");
}

#[test]
fn missing_argument_prints_usage() {
    let output = run(&[]);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("Usage: runlog <file.gpx>"));
}

#[test]
fn missing_file_fails_without_panic() {
    let path = fixture("does-not-exist.gpx");
    let output = run(&[path.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("Unable to open"));
}

#[test]
fn invalid_gpx_prints_parse_error() {
    let path = fixture("not-gpx.txt");
    let output = run(&[path.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("Unable to parse gpx in"));
}

#[test]
fn gpx_without_tracks_fails() {
    let path = fixture("no-tracks.gpx");
    let output = run(&[path.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("GPX file contains no tracks"));
}

#[test]
fn track_without_segments_fails() {
    let path = fixture("no-segments.gpx");
    let output = run(&[path.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("Track contains no segments"));
}

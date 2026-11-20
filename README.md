# runlog
A small CLI tool to track your runs.

This project is intended as a personal learning project to teach myself some Rust programming.

## Requirements

- Rust
- Cargo

## Installation

1. Clone the project
```bash
  git clone https://github.com/tcvscheppingen/runlog.git
  cd runlog
```

### Build `runlog`
```bash
  make build
```

Or:

```bash
  cargo build --release
```

### Install the build:

```bash
  sudo make install
``` 

To uninstall `runlog` use the following command:

```bash
  sudo make uninstall
```

## Development

Enable the git hooks so formatting, clippy and the tests run before every commit:

```bash
  make hooks
```

## Usage

> Use `runlog -h` to view complete usage information.

Use `runlog` with a gpx file you created. You can create one with a website like: [How far did I run?](https://onthegomap.com/#/create).

Calculate the distance of a route and the average pace based on a specified duration:

```bash
  runlog pace -r name-of-gpx-file.gpx -d HH:MM:SS # Replace HH:MM:SS with the duration of the run.
```

The duration input can be used as follows:

- `HH:MM:SS` for example: `02:35:36`, where the leading `0` can be left out.
- `MM:SS` for example: `35:36`.
- `MM` for example: `35`.

You can also just calculate the distance of a route without the pace:

 ```bash
 runlog dist name-of-gpx-file.gpx
 ```

 ## Keeping a running log

 You can use runlog to keep track of your running training in a spreadsheet.  
 Use the following command to initialize a running log:

 ```bash
runlog init name-of-running-log.csv
 ```

You can add records to the log with the following command:

```bash
runlog add --route path/to/gpx-file.gpx --duration 2:35:36 name-of-running-log.csv
```
You can leave the name of the running log blank to use the default `running-log.csv`

# runlog
A small CLI tool to track your runs.

This project is intended as a personal learning project to teach myself some Rust programming.

### Requirements

- Rust
- Cargo

### Installation

1. Clone the project
```bash
  git clone https://github.com/tcvscheppingen/runlog.git
  cd runlog
```

2. Build `runlog`
```bash
  cargo build --release
```

Or:

```bash
  make build
```

3. Install the build:

```bash
  sudo make install
``` 

To uninstall `runlog` use the following command:

```bash
  sudo make uninstall
```

### Usage

> Use `runlog -h` to view usage information.

Use `runlog` with a gpx file you created. You can create one with a website like: [How far did I run?](https://onthegomap.com/#/create).

As of now `runlog` can only calculate the average pace and distance of your run, more features will be added.

```bash
  runlog -r name-of-gpx-file.gpx -d HH:MM:SS # Replace HH:MM:SS with the duration of the run.
```

The duration input can be used as follows:

- `HH:MM:SS` for example: `02:35:36`, where the leading `0` can be left out.
- `MM:SS` for example: `35:36`.
- `MM` for example: `35`.

You can also just calculate the distance of a route without the pace:

 ```bash
 runlog -r name-of-gpx-file.gpx
 ```

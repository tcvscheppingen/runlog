# runlog
A small CLI tool to keep a running log in a file.

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
  cargo build
```

3. The build is in the target folder, you can move it to `~/.local/bin/` to make it accessible from the terminal.

### Usage

Use `runlog` with a gpx file you created. You can created one with a website like: [How far did I run?](https://onthegomap.com/#/create)

As of now `runlog` can only calculate the average pace and distance of your run

```bash
  runlog name-of-gpx-file.gpx MM:SS # Replace MM:SS with the duration of the run in minutes and seconds.
```




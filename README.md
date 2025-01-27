<h1 align="center">bunyan-rs</h1>
<div align="center">
 <strong>
   A Rust port of <a href="https://github.com/trentm/node-bunyan" target="_blank">node-bunyan</a>.
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/bunyan">
    <img src="https://img.shields.io/crates/v/bunyan.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/bunyan">
    <img src="https://img.shields.io/crates/d/bunyan.svg?style=flat-square"
      alt="Download" />
  </a>
</div>
<br/>

> _Structured logs are the greatest thing since sliced bread._

Are you annoyed from having to install `npm` just to get a copy of the amazing [NodeJS bunyan CLI](https://github.com/trentm/node-bunyan) to pretty-print your logs?

I feel you!

That's why I wrote `bunyan-rs`, a Rust port of (a subset of) the original [NodeJS bunyan CLI](https://github.com/trentm/node-bunyan).

<div>
<img src="https://raw.githubusercontent.com/LukeMathWalker/bunyan/main/images/ConsoleBunyanOutput.png" />
</div>
<hr/>

# Table of Contents
0. [How to install](#how-to-install)
1. [How to use](#how-to-use)
2. [Limitations](#limitations)
3. [Bunyan ecosystem in Rust](#bunyan-ecosystem-in-rust)
4. [Benchmarks](#benchmarks)
5. [License](#license)

## How to install

Using `cargo`:
```bash
cargo install bunyan
```

You can verify your installation with
```bash
bunyan --help
```

Alternatively, you can download a pre-built binary for your operating system from the [latest release](https://github.com/LukeMathWalker/bunyan/releases).

## How to use

`bunyan-rs` only supports stdin as input source.

You can pipe a log file into it:
```bash
cat tests/all/corpus/all.log | bunyan
```

Or you can pipe the output of a long-running job into it:
```bash
# Tail logs from a Docker container
docker logs -f my-app | bunyan

# Tail logs from a Kubernetes pod using kubectl
kubectl logs -f my-app-asdadf-cvcvcv

# Tail logs from a group of Kubernetes pods using stern
stern "my-app" --output raw --tail 100 | bunyan
```

## Limitations

Compared to the original `bunyan` CLI, `bunyan-rs`:

- Only supports `stdin` as input source (no files);
- Does not support log snooping via DTrace (`-p` argument);
- Does not support the `-c/--condition` filtering mechanism;
- Does not support the `--pager/--no-pager` flags;
- Only supports UTC format for time.

Some of the above might or might not be added in the future.
If you are interested in contributing, please open an issue.

## Bunyan ecosystem in Rust

You are writing a Rust application and you'd like to emit logs in bunyan format - what can you use?

Check out the following crates:

- [`tracing-bunyan-formatter`](https://crates.io/crates/tracing-bunyan-formatter), a bunyan formatter for [`tracing`](https://crates.io/crates/tracing);
- [`slog-bunyan`](https://crates.io/crates/slog-bunyan), a bunyan formatter for [`slog`](https://crates.io/crates/slog).

## Benchmarks

Speed has never been a burning problem while eyeballing logs from applications, but any speed-up to the tools I use on a daily basis is always appreciated.

To benchmark `bunyan-rs` against the original NodeJS `bunyan` follow these steps:

- Build `bunyan-rs` using the `release` profile:
```bash
cargo build --release
```
- Install `bunyan` via `npm`. You will need `npx` as well;
- Benchmark!
```bash
# bunyan JS
time ./benchmark_js.sh benchmark_logs.txt
# bunyan-rs
time ./benchmark_rs.sh benchmark_logs.txt
```

On my system `bunyan-rs` is roughly 5x faster on this very non-scientific and highly inaccurate benchmark - your mileage may vary.
The Rust code is highly non-optimised (we are allocating freely and wastefully!) - streamlining it could be a fun exercise.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

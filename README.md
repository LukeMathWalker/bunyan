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

# Table of Contents
0. [How to install](#how-to-install)
1. [How to use](#how-to-use)
2. [Limitations](#limitations)
3. [License](#license)

## How to install

You can either:

- Download a copy of the pre-built binary for your platform from the latest release on GitHub;
- Install from source using `cargo`:
```bash
cargo install bunyan
```

You can verify your installation with
```bash
bunyan --help
```

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
- Only supports the `long` output format;
- Only supports UTC format for time.

Some of the above might or might not be added in the future.  
If you are interested in contributing, please open an issue.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

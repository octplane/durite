# About

Very simple graphite disk free probe.

- written in rust
- needs df to run

Send all df visible `/something` partition disk space to graphite very 10s.

Metric is named:

`
durite.hostname.path.do.dev.{available/all}
`

# Build

## Host with rust nightly installed

`
cargo build --release
`

## Host without rust nightly

Uses docker.
`
make
`


# Usage

`
./target/release/durite -g graphite.hostna.me -l myhostname -p 2003
`


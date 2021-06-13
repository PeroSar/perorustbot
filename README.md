# perorustbot v0.1.0-dev
<img src="https://img.shields.io/badge/Version-v0.1.0%20(dev)-blue.svg">
<img src="https://img.shields.io/crates/v/perorustbot.svg">
[![CI](https://github.com/PeroSar/perorustbot/actions/workflows/rust.yml/badge.svg?branch=dev)](https://github.com/PeroSar/perorustbot/actions/workflows/rust.yml)
My personal telegram bot in rust

## Setting up your environment
 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 3. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows
$ set TELOXIDE_TOKEN=<Your token here>
```
 4. Make sure that your Rust compiler is up to date:
```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```


## Installation
 1. Build manually (recommended)
```bash
$ # clone this repo
$ # edit src/plugins/sudo.rs and replace my telegram id with yours
$ cargo run --release
```
 2. from crates.io
```bash
$ cargo install perorustbot
$ # make sure you have ~/.cargo/bin/ in your PATH
$ perorustbot
```

> list of commands
+ say
+ help (list of all commands)
+ ping
+ ipi (soon)
+ follow (soon)
+ cs (corona stats)
+ ani (anime info)
+ ctid (chat id, your id, replied users id)
+ udi (urban dict)

## This bot can be found at https://t.me/Pero_Rust_Bot



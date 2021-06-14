# perorustbot v0.1.0-dev
<img src="https://img.shields.io/badge/Version-v0.1.0%20(dev)-blue.svg">
<a href="https://crates.io/crates/perorustbot">
<img src="https://img.shields.io/crates/v/perorustbot.svg">
<a href="https://github.com/PeroSar/perorustbot/actions">
<img src="https://github.com/PeroSar/perorustbot/actions/workflows/rust.yml/badge.svg?branch=dev">
<a href="https://crates.io/crates/perorustbot">
<img src="https://img.shields.io/crates/d/perorustbot?color=brightgreen&label=crates.io%20downloads&logo=rust">
<a href="https://t.me/bots_rs">
<img src="https://img.shields.io/endpoint?style=flat&url=https%3A%2F%2Frunkit.io%2Fdamiankrawczyk%2Ftelegram-badge%2Fbranches%2Fmaster%3Furl%3Dhttps%3A%2F%2Ft.me%2Fbots_rs">
## Description
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
$ git clone https://github.com/PeroSar/perorustbot
$ cd perorustbot
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
+ start
+ help (list of all commands)
+ ping
+ ipi (soon)
+ follow (soon)
+ cs (corona stats)
+ ani (anime info)
+ ctid (chat id, your id, replied users id)
+ udi (urban dict)
+ upload 

## This bot can be found at https://t.me/Pero_Rust_Bot



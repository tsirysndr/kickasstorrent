# KickassTorrents CLI

<p>
  <a href="https://crates.io/crates/kickasstorrent" target="_blank">
    <img src="https://img.shields.io/crates/v/kickasstorrent.svg" />
  </a>
   <a href="https://crates.io/crates/kickasstorrent" target="_blank">
    <img src="https://img.shields.io/crates/dr/kickasstorrent" />
  </a>
  <a href="https://docs.rs/kickasstorrent" target="_blank">
    <img src="https://docs.rs/kickasstorrent/badge.svg" />
  </a>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
  <a href="https://github.com/tsirysndr/kickasstorrent/actions/workflows/release.yml" target="_blank">
    <img alt="License: MIT" src="https://github.com/tsirysndr/kickasstorrent/actions/workflows/release.yml/badge.svg" />
  </a>
  <a href="https://github.com/tsirysndr/kickasstorrent/actions/workflows/rust-clippy.yml" target="_blank">
    <img alt="release" src="https://github.com/tsirysndr/kickasstorrent/actions/workflows/rust-clippy.yml/badge.svg?branch=master" />
  </a>
</p>

<p>
<a href="https://www.buymeacoffee.com/tsiry">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-red.png" alt="Buy Me A Coffee" height="40" />
</a>
</p>

KickassTorrents CLI is a command line interface and Rust library for KickassTorrents.

<img width="800" src="./preview.svg">

## Installation

```bash
cargo install kickasstorrent
```

### macOS/Linux

```bash
brew install tsirysndr/tap/kickasstorrent
```

Or download the latest release for your platform from [here](https://github.com/tsirysndr/kickasstorrent/releases).

## Usage

```

USAGE:
    kickasstorrent <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    category           List torrents in a category
    help               Print this message or the help of the given subcommand(s)
    info               Show information about a torrent
    latest             Show latest torrents
    latest-searches    Show latest searches
    popular            Show popular torrents
    search             Search for torrents

```

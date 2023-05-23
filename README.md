# rustoku-cli
A lightweight CLI remote application for roku's.  Inspired by other similar roku tui applications, rustoku-cli uses the internal [External Control Protocol (ECP)](https://developer.roku.com/en-ca/docs/developer-program/dev-tools/external-control-api.md) provided by roku's API.  By sending empty post requests to certain URI's you can turn your computer into a remote!

This project is for personal use and a introduction to learning rust.  But feel free to send a [pull request](https://github.com/shee0049/rustoku-cli/pulls)

## Pre installation

Before running this application make sure to do a couple of things.

1. Download and install rust https://rustup.rs/
2. Determine the IP address of your roku, this can usually be found in the network settings.

### Installation

Clone this repository

```bash
git clone https://github.com/shee0049/rustoku-cli/
``` 

```bash 
cargo run
```



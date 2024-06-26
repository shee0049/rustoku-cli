# rustoku-cli
A lightweight CLI remote application to control your roku using your terminal and VIM keys!  Inspired by other similar tui applications, rustoku-cli uses the [External Control Protocol (ECP)](https://developer.roku.com/en-ca/docs/developer-program/dev-tools/external-control-api.md) provided by roku's API.  By sending empty POST requests to certain URI's you can turn your computer into a remote!

This project was created for my own personal use and as an introduction to learning rust.  But feel free to send a [pull request](https://github.com/shee0049/rustoku-cli/pulls) if you would like to add anything.

### Pre-installation

Before running this application make sure to do a couple of things:

 - Download and install rust https://rustup.rs/
 - Determine the IP address of your roku, this can usually be found in the network settings.

### Installation

Clone this repository

```bash
git clone https://github.com/shee0049/rustoku-cli/ && cd rustoku-cli
``` 

Edit the IP_ADDR constant in main.rs with your roku's IP address

run rustoku-cli

```bash 
cargo run
```

### Create a binary

Once you have ran the project, you can create a binary so you can run it directly from the command line

```bash
cargo build --release
```

this will create the binary file we just need to move it over to somewhere in your system $PATH

```bash
sudo cp ./target/release/rustoku-cli /usr/local/bin/
```


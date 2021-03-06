# XRPL Hook Template

_*Before you begin, it is highly recommended that you read 
the [official docs](https://xrpl-hooks.readme.io/) carefully._

This bootstrap template allows you to write XRPL hooks in Rust.

You may need:

- [Examples](examples) to view use cases

- [Rust Hooks Api Documentation](https://docs.rs/xrpl-hooks/)

- [Hooks Testnet](https://hooks-testnet.xrpl-labs.com/) where you can create test accounts, view transactions, etc.

# XRPL Hackathon 2021

This is part of the [XRPL Hooks Rust](https://devpost.com/software/xrpl-hooks-rust) project developed for [XRPL Hackathon 2021](https://xrpl-hackathon-2021.devpost.com/).

![](XRPL2021HACK.jpg)

# Prerequisites

1. We need `rustup` if not already installed

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.bashrc
```

2. Add wasm32 target

```bash
rustup target add wasm32-unknown-unknown
```


3. We need `nodejs v12` for setting hooks to XRPL

```bash
curl -fsSL https://deb.nodesource.com/setup_12.x | sudo -E bash -
sudo apt-get install -y nodejs
```

# Clone

Clone this repo

```bash
git clone https://github.com/otov4its/xrpl-hook-template.git
cd xrpl-hook-template/
```

# Workflow

## Step 0
Make changes in `src/lib.rs`

## Step 1

Build

```bash
./build
```

The builded wasm hook is contained in the project root directory.

## Step 2

Set hook

```bash
nodejs set_hook s*** hook_debug
```

## Step 3
Check

```bash
nodejs pay s*** 1000 r***
```

## Step 4

Goto -> "Step 0" :)

## Release

```bash
nodejs set_hook s*** hook
```

# Building examples

```bash
./build_examples
```
The builded wasm are contained in [examples](examples) folder.

To set example hook run:

```bash
nodejs set_hook s*** examples/HOOKNAME
```

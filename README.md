# XRPL Hook Template

This bootstrap template allows you to write XRPL hooks in Rust.

To view examples see [examples](examples) folder.

Before you begin, it is highly recommended that you read 
the [official docs](https://xrpl-hooks.readme.io/) carefully.

[Rust Api Documentation](https://docs.rs/xrpl-hooks/)

# XRPL Hackathon 2021

The hook template was written as part of [XRPL Hackathon 2021](https://xrpl-hackathon-2021.devpost.com/)

![](XRPL2021HACK.jpg)

# Prerequisites

1. We need `rustup` if not already installed
```bash
    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
    $ source ~/.bashrc
```

2. Add wasm32 target

```bash
    $ rustup target add wasm32-unknown-unknown
```


3. We need `nodejs v12` for setting hooks to XRPL

```bash
    $ curl -fsSL https://deb.nodesource.com/setup_12.x | sudo -E bash -
    $ sudo apt-get install -y nodejs
```

# Clone

Clone this repo

```bash
$ git clone https://github.com/otov4its/xrpl-hook-template.git
$ cd xrpl-hook-template/
```

# Workflow

## Step 0
Make changes in `src/lib.rs`

## Step 1
Build
```bash
$ ./build
```

## Step 2
Set
```bash
$ nodejs set_hook s*** hook_debug
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
$ nodejs set_hook s*** hook
```

# Building examples

```bash
$ ./build_examples
```
The builded wasm are contained in [examples](examples) folder

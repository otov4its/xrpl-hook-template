# XRPL Hackathon 2021

The hook template was written as part of [XRPL Hackathon 2021](https://xrpl-hackathon-2021.devpost.com/)

![](XRPL2021HACK.jpg)

# Workflow

## Step 0
Add wasm target
```bash
$ rustup target add wasm32-unknown-unknown
```

## Step 1
Clone
```bash
$ git clone https://github.com/otov4its/xrpl-hook-template.git
```

## Step 2
Make changes in `src/lib.rs`

## Step 3
Build
```bash
$ ./build
```

## Step 4
Set
```bash
$ node set_hook s*** hook_debug
```

## Step 5
Check
```bash
node pay s*** 1000 r***
```

## Step 6

Goto -> "Step 2" :)

## Release
```bash
$ node set_hook s*** hook
```

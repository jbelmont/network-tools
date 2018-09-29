# Network Tools

A library of network tools written in Rustlang

## IP Address Util

You can run the ip_address script by doing the following as an example:

```bash
cargo run -p ip_address en0
```

Here I am running the en0 interface port for Mac OS X

This will print out my current IP ADDRESS.

## pwd Rust equivalent Util

You can run the pwd util script like this:

```bash
cargo run -p pwd
```

## ls Rust Equivalent Util

You can run the ls util script like this:

```bash
cargo run -p ls
```

#### Running Make Tasks

You can also equivalently run each util script with the Makefile like this:

```bash
make util UTIL=ls
```

This will will run the ls script!

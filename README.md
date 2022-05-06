<h1 align="center">kill-port</h1>
<div align="center">
  <strong>Kill the process running on given port</strong>
</div>

## Install

```sh
$ git clone https://github.com/robberphex/kill-port.git
$ cargo build --release
# add ./target/release/kill-port to PATH.
```

## Usage

```js
kill-port 0.1.0

USAGE:
    kill-port [PORT]...

ARGS:
    <PORT>...    ports to find

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Example

```sh
$ kill-port 8080
```

You can also kill multiple ports:

```sh
$ kill-port 8080,5000,3000
# OR
$ kill-port 9000 3000 5000
```

## Contribute

Contributions are welcome. Please open up an issue or create PR if you would like to help out.

## License

Licensed under Apache License Version 2.0

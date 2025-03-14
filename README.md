# pigen-tcp

Happy &pi; day!

This demo project is a Rust implementation of a slightly modified TCP-based Pigen protocol specified in [IETF RFC 3091](https://datatracker.ietf.org/doc/html/rfc3091). The main differences between the official specification and this implementation are:

- use of TCP port 31415 instead of invalid 314159
- explicit transmission of the first digit of pi before the decimal point

## Run in dev mode

```
cargo run
```

## Build

``` 
cargo build --release
```

## License

GNU General Public License v3

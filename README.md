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

## Acknowledgements

Special thanks go to the authors of the [`threadpool`](https://crates.io/crates/threadpool) and [`rug`](https://crates.io/crates/rug) crates that power this app, as well as [_The ABC Programmer's Handbook_](https://homepages.cwi.nl/~steven/abc/programmers/examples.html) for providing the &pi; digit spigot algorithm.

## License

GNU General Public License v3

# pigen-tcp

Happy &pi; day!

This demo project is a Rust implementation of a slightly modified TCP-based Pigen protocol specified in [IETF RFC 3091](https://datatracker.ietf.org/doc/html/rfc3091). The main differences between the official specification and this implementation are:

- use of TCP port 31415 instead of invalid 314159
- explicit transmission of the first digit of pi before the decimal point

## Running in dev mode

```
cargo run
```

## Building

``` 
cargo build --release
```

## Connecting to the server

You can write a custom TCP client or use a general-purpose one such as [`ncat`](https://nmap.org/ncat):

```
ncat --recv-only <SERVER_IP> 31415
```

## Acknowledgements

Special thanks go to the authors of the [`threadpool`](https://crates.io/crates/threadpool) and [`rug`](https://crates.io/crates/rug) crates that power this app, as well as [_The ABC Programmer's Handbook_](https://homepages.cwi.nl/~steven/abc/programmers/examples.html) for providing the &pi; digit spigot algorithm.

## License

GNU General Public License v3

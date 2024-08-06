# psrsee

psrsee outputs a process tree in JSON machine-readable format.

written in Rust because we are in 2024.

## Build

```
$ cargo build
```

## Run

```
./psrsee
```
```json
{"pid":"1","cmdline":"/sbin/init","uid":["0","0","0","0"],"gid":["0","0","0","0"],"children":[{"pid":"1888735","cmdline":"/usr/bin/atop -w /var/log/atop/atop_20240806 600","uid":["0","0","0","0"],"gid":["0","0","0","0"],"children":[]}}
```

## License

MIT

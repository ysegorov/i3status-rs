# i3status

Utility to generate status bar data to be used by [i3][i3] window manager bar
(known as [i3bar][i3bar]).

Serves as a replacement for [i3status][i3status].

Implements [i3bar protocol][i3bar-protocol].

**Written in Rust as educational project for personal use only.**

Check [greshake/i3status-rust][i3status-rust] for full featured `i3status`
implementation written in Rust.


## Runtime Dependencies

- Linux
- mounted `sysfs` filesystem
- `iwconfig`, `iwgetid` (`wireless-tools` package)
- `ip` (`iproute2` package)
- `vmstat` (`procps-ng` package)
- `xkblayout-state` (`xkblayout-state-git` package)
- `amixer` (`alsa-utils` package)
- `df` (`coreutils` package)


## Usage

Install [Rust][rust-install] to build binary.

Clone this repository, build and install binary:

```sh
$ git clone https://github.com/ysegorov/i3status-rs
$ cd i3status-rs
$ cargo install --path .
```

Binary named `i3status` will be installed to `$CARGO_HOME/bin` directory
(`$CARGO_HOME` defaults to `~/.cargo`).

Configure `i3bar` within `~/.config/i3/status` configuration file:

```
...
bar {
    status_command ~/.cargo/bin/i3status
    font pango:Hack Nerd Font 9
}
...
```

and reload `i3wm` configuration or simply restart `i3wm`.


## License

[Unlicense][unlicense].


[i3]: https://i3wm.org/
[i3bar]: https://i3wm.org/docs/userguide.html#_configuring_i3bar
[i3status]: https://i3wm.org/i3status/manpage.html
[i3bar-protocol]: https://i3wm.org/docs/i3bar-protocol.html
[rust-install]: https://www.rust-lang.org/tools/install
[unlicense]: http://unlicense.org
[i3status-rust]: https://github.com/greshake/i3status-rust

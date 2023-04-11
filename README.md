# system_tz

[![Version](https://img.shields.io/crates/v/system_tz.svg)](https://crates.io/crates/system_tz)
[![Documentation](https://img.shields.io/docsrs/system_tz)](https://docs.rs/system_tz)
[![License](https://img.shields.io/crates/l/system_tz.svg)](https://github.com/b4D8/system_tz/blob/main/LICENSE)

This utility crate provides a single trait `SystemTz` which exposes the `system_tz()`
method allowing to get the [timezone](https://en.wikipedia.org/wiki/Time_zone)
from the operating system.

Should support the following operating system families: `unix`, `windows` and `wasm`.

Effectively tested on:
- 2023-04-11: Debian GNU/Linux 11 (bullseye)
- 2023-04-11: Microsoft Windows 11
- ...

Valid timezones are represented with [`chrono_tz::Tz`](https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html)
based on [IANA Time Zone Database](https://www.iana.org/time-zones) (Olson names).

On Microsoft Windows, because it uses of a special naming convention,
the method relies on [`WindowsZones`](https://github.com/unicode-org/cldr/blob/main/common/supplemental/windowsZones.xml),
a dataset maintained by the [Unicode Common Locale Data Repository (CLDR)](https://cldr.unicode.org/),
which is downloaded and built into a static global object during compilation.

## Safety

Attention was given to provide safe implementation (no `unwrap()` or `expect()`)
but note that on windows target:
* the build script is faillible (it is designed to panic on error)
* 1 `unsafe` is used by the _fallback_ method because of [`windows`](https://crates.io/crates/windows) API.

## Command-line interface

The crate provides a very basic binary which will print the system timezone on invokation.

### Installation

Refer to the [official documentation](https://www.rust-lang.org/learn/get-started)
for installing the `cargo` package manager, and then run the following command
from a terminal:

```bash
$ cargo install system_tz
```

### Usage

The interface doesn't require any argument.

```bash
$ tz
Europe/Paris
```

## Contribute

Contributions to the project are most welcome.
In particular, **please let us known whether it works on your device**
so we can improve the implementation and extend support.

Pull request imply agreement to the [Developer's certificate of origin (`DCO-1.1`)](https://developercertificate.org/).

## Credits

* [tzlocal](https://github.com/regebro/tzlocal) (MIT)
* [localzone](https://github.com/mitsuhiko/localzone) (Apache-2.0).

# system_tz

[![Version](https://img.shields.io/crates/v/system_tz.svg)](https://crates.io/crates/l/system_tz)
[![Documentation](https://img.shields.io/docsrs/system_tz)](https://docs.rs/system_tz)
[![License](https://img.shields.io/crates/l/system_tz.svg)](https://crates.io/crates/l/system_tz)

This utility crate provides a single trait `SystemTz` which exposes the `system_tz()`
method allowing to get the [timezone](https://en.wikipedia.org/wiki/Time_zone)
from the operating system.

Currently supported operating system families include `unix` (Linux, MacOs), `windows` and `wasm`.

Valid timezones are represented with [`chrono_tz::Tz`](https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html) based on [IANA Time Zone Database](https://www.iana.org/time-zones) (Olson names).

On Microsoft Windows, because it uses of a special naming convention,
the method relies on [`WindowsZones`](https://github.com/unicode-org/cldr/blob/main/common/supplemental/windowsZones.xml),
a dataset maintained by the [Unicode Common Locale Data Repository (CLDR)](https://cldr.unicode.org/),
which is downloaded and built into a static global object during compilation.

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
$ system_tz
Europe/Paris
```

## Contribute

Contributions to the project are most welcome.
In particular, **please let us known whether it works on your device**
so we can improve the implementation.

Pull request imply agreement to the [Developer's certificate of origin (`DCO-1.1`) ](https://developercertificate.org/).

## Credits

* [tzlocal](https://github.com/regebro/tzlocal) (MIT)
* [localzone](https://github.com/mitsuhiko/localzone) (Apache-2.0).

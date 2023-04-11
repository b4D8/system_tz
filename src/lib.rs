/******************************************************************************
    Copyright 2022 b4D8

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*******************************************************************************/

//! [![Version](https://img.shields.io/crates/v/system_tz.svg)](https://crates.io/crates/system_tz)
//! [![Documentation](https://img.shields.io/docsrs/system_tz)](https://docs.rs/system_tz/latest/system_tz)
//! [![License](https://img.shields.io/crates/l/system_tz.svg)](https://github.com/b4D8/system_tz/blob/main/LICENSE)
//!
//! This utility crate provides a single trait `SystemTz` which exposes the `system_tz()`
//! method allowing to get the [timezone](https://en.wikipedia.org/wiki/Time_zone)
//! from the operating system.
//!
//! Should support the following operating system families: `unix`, `windows` and `wasm`.
//!
//! Effectively tested on:
//! - 2023-04-11: Debian GNU/Linux 11 (bullseye)
//! - 2023-04-11: Microsoft Windows 11
//! - ...
//!
//! Valid timezones are represented with [`chrono_tz::Tz`](https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html)
//! based on [IANA Time Zone Database](https://www.iana.org/time-zones) (Olson names).
//!
//! On Microsoft Windows, because it uses of a special naming convention,
//! the method relies on [`WindowsZones`](https://github.com/unicode-org/cldr/blob/main/common/supplemental/windowsZones.xml),
//! a dataset maintained by the [Unicode Common Locale Data Repository (CLDR)](https://cldr.unicode.org/),
//! which is downloaded and built into a static global object during compilation.
//!
//! ## Safety
//!
//! Attention was given to provide safe implementation (no `unwrap()` or `expect()`)
//! but note that on windows target:
//! * the build script is faillible (it is designed to panic on error)
//! * 1 `unsafe` is used by the _fallback_ method because of [`windows`](https://crates.io/crates/windows) API.
//!
//! ## Command-line interface
//!
//! The crate provides a very basic binary which will print the system timezone on invokation.
//!
//! ### Installation
//!
//! Refer to the [official documentation](https://www.rust-lang.org/learn/get-started)
//! for installing the `cargo` package manager, and then run the following command
//! from a terminal:
//!
//! ```bash
//! $ cargo install system_tz
//! ```
//!
//! ### Usage
//!
//! The interface doesn't require any argument.
//!
//! ```bash
//! $ tz
//! Europe/Paris
//! ```
//!
//! ## Contribute
//!
//! Contributions to the project are most welcome.
//! In particular, **please let us known whether it works on your device**
//! so we can improve the implementation and extend support.
//!
//! Pull request imply agreement to the [Developer's certificate of origin (`DCO-1.1`)](https://developercertificate.org/).
//!
//! ## Credits
//!
//! * [tzlocal](https://github.com/regebro/tzlocal) (MIT)
//! * [localzone](https://github.com/mitsuhiko/localzone) (Apache-2.0).

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use chrono_tz::Tz;

#[cfg(test)]
mod test;

/// Abstract method for timezone retreival from the current operating system.
pub trait SystemTz {
    #[must_use]
    /// Tries to get a [`Tz`] from the operating system.
    fn system_tz() -> Option<Tz>;
}

trait AsTz {
    #[must_use]
    /// Tries to cast type to [`Tz`]
    fn as_tz(&self) -> Option<Tz>;
}

impl<T: AsRef<str>> AsTz for T {
    /// Tries to parse a `Tz`.
    fn as_tz(&self) -> Option<Tz> {
        Tz::from_str_insensitive(self.as_ref().trim()).ok()
    }
}

// UNIX ////////////////////////////////////////////////////////////////////////

#[cfg(target_family = "unix")]
impl<T: chrono::TimeZone> SystemTz for T {
    fn system_tz() -> Option<Tz> {
        use ::std::{env, fs};

        env::var("TZ")
            .ok()
            .and_then(|tz| tz.as_tz())
            .or_else(|| {
                fs::read_to_string("/etc/timezone")
                    .ok()
                    .and_then(|tz| tz.as_tz())
            })
            .or_else(|| {
                fs::read_to_string("/var/db/zoneinfo")
                    .ok()
                    .and_then(|tz| tz.as_tz())
            })
            .or_else(|| {
                // References:
                // * https://man7.org/linux/man-pages/man5/localtime.5.html
                // * https://www.man7.org/linux/man-pages/man1/timedatectl.1.html
                fs::read_link("/etc/localtime")
                    .ok()
                    .and_then(|x| x.canonicalize().ok())
                    .and_then(|x| {
                        x.display()
                            .to_string()
                            .split_once("/zoneinfo/")
                            .and_then(|(_, tz)| tz.as_tz())
                    })
            })
            .or_else(|| {
                fs::read_link("usr/local/etc/localtime")
                    .ok()
                    .and_then(|x| x.canonicalize().ok())
                    .and_then(|x| {
                        x.display()
                            .to_string()
                            .split_once("/zoneinfo/")
                            .and_then(|(_, tz)| tz.as_tz())
                    })
            })
            .or_else(|| {
                // CentOS and OpenSUSE
                fs::read_to_string("etc/sysconfig/clock")
                    .ok()
                    .and_then(|info| {
                        info.lines()
                            .find(|line| {
                                let line = line.trim_start();
                                line.starts_with("ZONE") || line.starts_with("TIMEZONE")
                            })
                            .and_then(|line| line.split_once('=').and_then(|(_, tz)| tz.as_tz()))
                    })
            })
            .or_else(|| {
                // Gentoo
                fs::read_to_string("/etc/conf.d/clock")
                    .ok()
                    .and_then(|info| {
                        info.lines()
                            .find(|line| line.trim_start().starts_with("TIMEZONE"))
                            .and_then(|line| line.split_once('=').and_then(|(_, tz)| tz.as_tz()))
                    })
            })
            .or_else(|| {
                fs::read_to_string("/etc/default/init")
                    .ok()
                    .and_then(|info| {
                        info.lines()
                            .find(|line| line.trim_start().starts_with("TZ"))
                            .and_then(|line| line.split_once('=').and_then(|(_, tz)| tz.as_tz()))
                    })
            })
            .or_else(|| {
                fs::read_to_string("usr/local/etc/default/init")
                    .ok()
                    .and_then(|info| {
                        info.lines()
                            .find(|line| line.trim_start().starts_with("TZ"))
                            .and_then(|line| line.split_once('=').and_then(|(_, tz)| tz.as_tz()))
                    })
            })
    }
}

// WINDOWS /////////////////////////////////////////////////////////////////////

#[cfg(target_family = "windows")]
include!(concat!(env!("OUT_DIR"), "/windows_zones.rs"));

#[cfg(target_family = "windows")]
trait WindowsUtf16 {
    #[must_use]
    /// Tries to cast Windows UTF-16 to valid UTF-8.
    fn as_utf8(&self) -> Option<String>;
}

#[cfg(target_family = "windows")]
impl WindowsUtf16 for [u16; 32] {
    fn as_utf8(&self) -> Option<String> {
        Some(String::from_utf16_lossy(self.split(|x| *x == 0).next()?))
    }
}

#[cfg(target_family = "windows")]
impl WindowsUtf16 for [u16; 128] {
    fn as_utf8(&self) -> Option<String> {
        Some(String::from_utf16_lossy(self.split(|x| *x == 0).next()?))
    }
}

#[cfg(target_family = "windows")]
#[derive(Debug, Clone, Copy, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Errors of this crate.
pub enum Error {
    #[error("Unknown timezone")]
    UnknownTimezone,
}

#[cfg(target_family = "windows")]
struct WindowsZonesVersion {
    pub build_date: Option<chrono::DateTime<chrono::Utc>>,
    pub version: (&'static str, &'static str),
    pub hash: Option<u64>,
}

#[cfg(target_family = "windows")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Known Microsoft Windows timezone.
pub struct WindowsTz {
    zone: &'static str,
    territory: Option<&'static str>,
    iana: Vec<&'static str>,
}

#[cfg(target_family = "windows")]
impl WindowsTz {
    #[must_use]
    /// Returns a `WindowsTz` **only if it is registered in `WindowsZones` dataset**.
    ///
    /// If no `territory` is provided, returns the first known `WindowsTz`,
    /// with a matching the `zone`.
    pub fn get(zone: &str, territory: Option<&str>) -> Option<&'static Self> {
        WINDOWS_ZONES.iter().find(|x| {
            let zone = x.zone == zone;
            if territory.is_some() {
                zone && x.territory == territory
            } else {
                zone
            }
        })
    }

    #[must_use]
    /// Returns the build date of the bundled `WindowsZones` dataset.
    pub fn build_date() -> Option<chrono::DateTime<chrono::Utc>> {
        WINDOWS_ZONES_VERSION.build_date
    }

    #[must_use]
    /// Returns the hash of the bundled `WindowsZones` dataset.
    pub fn hash() -> Option<u64> {
        WINDOWS_ZONES_VERSION.hash
    }

    #[must_use]
    /// Returns the version of the bundled `WindowsZones` dataset.
    pub fn version() -> (&'static str, &'static str) {
        WINDOWS_ZONES_VERSION.version
    }
}

#[cfg(target_family = "windows")]
impl TryFrom<&WindowsTz> for Tz {
    type Error = Error;

    fn try_from(tz: &WindowsTz) -> Result<Self, Self::Error> {
        // This should be infaillible as timezone validity is checked while building data
        tz.iana[0].parse().map_err(|_| Error::UnknownTimezone)
    }
}

#[cfg(target_family = "windows")]
impl TryFrom<&Tz> for WindowsTz {
    type Error = Error;

    fn try_from(tz: &Tz) -> Result<Self, Self::Error> {
        WINDOWS_ZONES
            .iter()
            .find(|x| x.iana.contains(&tz.name()))
            .cloned()
            .ok_or(Error::UnknownTimezone)
    }
}

#[cfg(target_family = "windows")]
impl<T: chrono::TimeZone> SystemTz for T {
    fn system_tz() -> Option<Tz> {
        use ::windows::{
            Globalization::Calendar,
            Win32::System::Time::{GetDynamicTimeZoneInformation, DYNAMIC_TIME_ZONE_INFORMATION},
        };

        Calendar::new()
            .ok()
            .and_then(|cal| {
                cal.GetTimeZone()
                    .ok()
                    .and_then(|hstring| hstring.to_string_lossy().as_tz())
            })
            .or_else(|| {
                // Reference: https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-gettimezoneinformation
                let mut zone_info = DYNAMIC_TIME_ZONE_INFORMATION::default();
                if let 0..=2 = unsafe { GetDynamicTimeZoneInformation(&mut zone_info) } {
                    zone_info.TimeZoneKeyName.as_utf8().and_then(|zone| {
                        WindowsTz::get(&zone, None)
                            .and_then(|windows_tz| windows_tz.try_into().ok())
                    })
                } else {
                    None
                }
            })
    }
}

// WASM ////////////////////////////////////////////////////////////////////////

#[cfg(target_family = "wasm")]
impl<T: chrono::TimeZone> SystemTz for T {
    fn system_tz() -> Option<Tz> {
        use {js_sys::Intl::DateTimeFormat, js_sys::Reflect};
        // Reference: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat
        let opts = DateTimeFormat::default().resolved_options();
        Reflect::get(&opts, &"timeZoneName".into())
            .ok()
            .and_then(|val| val.as_string().and_then(|s| s.as_tz()))
            .or_else(|| {
                Reflect::get(&opts, &"timeZone".into())
                    .ok()
                    .and_then(|val| val.as_string().and_then(|s| s.as_tz()))
            })
    }
}

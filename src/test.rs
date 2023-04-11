#[test]
#[cfg(target_family = "windows")]
fn is_handles_windows_tz() {
    use super::WindowsTz;

    let missing_windows_tz =
        "Timezone doesn't exist in latest version of `WindowsZones` CLDR dataset";

    assert_eq!(
        chrono_tz::Tz::from(
            WindowsTz::get("US Mountain Standard Time", Some("CA")).expect(missing_windows_tz)
        ),
        chrono_tz::America::Creston
    );

    assert_eq!(
        chrono_tz::Tz::from(
            WindowsTz::get("US Mountain Standard Time", None).expect(missing_windows_tz)
        ),
        chrono_tz::America::Phoenix
    );

    assert_eq!(
        WindowsTz::try_from(chrono_tz::Europe::Vienna).ok().as_ref(),
        WindowsTz::get("W. Europe Standard Time", Some("AT"))
    );

    let case = chrono_tz::Europe::Paris;
    let windows = WindowsTz::try_from(case).expect(missing_windows_tz);
    assert_eq!(case, chrono_tz::Tz::from(&windows));
}

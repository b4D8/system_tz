fn main() {
    use system_tz::SystemTz;
    if let Some(tz) = chrono_tz::Tz::system_tz() {
        println!("{tz}");
    } else {
        eprintln!("Error: Failed to get timezone");
        eprintln!(
            "You might want to report this error on {}",
            env!("CARGO_PKG_REPOSITORY")
        );
    }
}

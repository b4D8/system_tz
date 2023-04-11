#[cfg(windows)]
#[derive(serde::Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MapZone {
    #[serde(rename = "@other")]
    pub zone: String,
    #[serde(rename = "@territory")]
    pub territory: Option<String>,
    #[serde(rename = "@type")]
    pub iana: Vec<chrono_tz::Tz>,
}

#[cfg(windows)]
#[derive(serde::Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
struct MapTimezones {
    #[serde(rename = "@otherVersion")]
    other_version: String,
    #[serde(rename = "@typeVersion")]
    type_version: String,
    #[serde(rename = "$value")]
    zones: Vec<MapZone>,
}

#[cfg(windows)]
#[derive(serde::Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct WindowsZones {
    #[serde(rename = "$value")]
    timezones: MapTimezones,
}

#[cfg(windows)]
#[derive(serde::Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct WindowsZonesData {
    windows_zones: WindowsZones,
}

#[cfg(windows)]
impl WindowsZonesData {
    const SOURCE: &'static str = "https://raw.githubusercontent.com/unicode-org/cldr/main/common/supplemental/windowsZones.xml";

    /// Download latest dataset from `Self::SOURCE`.
    async fn get() -> Self {
        let request = reqwest::get(Self::SOURCE)
            .await
            .expect("Failed to GET Unicode CLDR data");

        let response = request
            .text()
            .await
            .expect("Failed to decode UTF-8 from HTTP response");

        let mut data: Self =
            quick_xml::de::from_str(&response).expect("Failed to deserialize XML data");

        for tz in [MapZone {
            zone: "Coordinated Universal Time".into(),
            territory: None,
            iana: vec![chrono_tz::Etc::UTC],
        }] {
            data.windows_zones.timezones.zones.push(tz)
        }

        data
    }

    /// Returns the hash of the downloaded dataset.
    fn hash(&self) -> u64 {
        let mut state = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&self, &mut state);
        std::hash::Hasher::finish(&state)
    }

    /// Writes the `WINDOWS_ZONES_VERSION` static containing metadata regarding build and dataset.
    fn _write_version(&self, f: &mut std::io::BufWriter<std::fs::File>) {
        use std::io::Write;
        let msg = "Failed to write version to `BufWriter`";

        //writeln!(f, "#[cfg(windows)]").expect(msg);
        writeln!(f, "/// Version of the bundled CLDR `WindowsZones` dataset").expect(msg);
        writeln!(
            f,
            "static WINDOWS_ZONES_VERSION: once_cell::sync::Lazy<WindowsZonesVersion> = once_cell::sync::Lazy::new(|| {{"
        )
        .expect(msg);
        writeln!(f, "   WindowsZonesVersion {{",).expect(msg);
        writeln!(
            f,
            "       build_date: {:?}.parse().expect(\"Invaild timestamp\"),",
            chrono::Utc::now().to_rfc3339()
        )
        .expect(msg);
        writeln!(
            f,
            "       version: ({:?}, {:?}),",
            &self.windows_zones.timezones.other_version, &self.windows_zones.timezones.type_version,
        )
        .expect(msg);
        writeln!(
            f,
            "       hash: \"{}\".parse().expect(\"Invalid hash integer\")",
            self.hash()
        )
        .expect(msg);
        writeln!(f, "   }}",).expect(msg);
        writeln!(f, "}});",).expect(msg);
        writeln!(f).expect(msg);
    }

    /// Writes a `WINDOWS_ZONES` static containing the downloaded data.
    fn _write_data(&self, f: &mut std::io::BufWriter<std::fs::File>) {
        use std::io::Write;
        let msg = "Failed to write data to `BufWriter`";

        //writeln!(f, "#[cfg(windows)]").expect(msg);
        writeln!(
            f,
            "/// Simplified representation of CLDR `WindowsZones` data"
        )
        .expect(msg);
        writeln!(f, "static WINDOWS_ZONES: once_cell::sync::Lazy<Vec<WindowsTz>> = once_cell::sync::Lazy::new(|| {{").expect(msg);
        writeln!(f, "   vec![").expect(msg);
        for MapZone {
            zone,
            territory,
            iana,
        } in &self.windows_zones.timezones.zones
        {
            writeln!(f, "       WindowsTz {{").expect(msg);
            writeln!(f, "           zone: {zone:#?},").expect(msg);
            writeln!(f, "           territory: {territory:?},").expect(msg);
            writeln!(f, "           iana: vec![").expect(msg);
            for tz in iana {
                writeln!(f, "               {:#?},", tz.name()).expect(msg);
            }
            writeln!(f, "           ]").expect(msg);
            writeln!(f, "       }},").expect(msg);
        }
        writeln!(f, "    ]").expect(msg);
        writeln!(f, "}});").expect(msg);
        writeln!(f).expect(msg);
    }

    /// Writes downloaded data to `path`.
    fn build<P: AsRef<std::path::Path>>(self, path: P) {
        let out_dir = std::env::var("OUT_DIR").expect("Failed to get `OUT_DIR` env variable");
        let out_path = std::path::Path::new(&out_dir).join(path.as_ref());
        let target = std::fs::File::create(out_path).expect("Failed to create file");
        let mut f = std::io::BufWriter::new(target);
        self._write_version(&mut f);
        self._write_data(&mut f);
    }
}

#[cfg(windows)]
#[tokio::main]
async fn main() {
    WindowsZonesData::get().await.build("windows_zones.rs")
}

#[cfg(not(windows))]
fn main() {}
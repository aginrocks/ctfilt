use serde::{Deserialize, Deserializer, Serializer};

#[cfg(feature = "readwrite")]
use {color_eyre::eyre::Result, url::Url};

#[cfg(feature = "readwrite")]
pub fn serialize_with_schema<T: serde::Serialize>(
    value: T,
    strust_name: &str,
    server_base: Url,
) -> Result<String> {
    let schema_url = server_base.join(&format!("/api/schema/{strust_name}"))?;
    let schema_comment = format!("# yaml-language-server: $schema={schema_url}\n");
    let serialized = serde_yaml::to_string(&value)?;

    let result = format!("{schema_comment}{serialized}");
    Ok(result)
}

/// Deserialize Option<i32> from octal string
pub fn deserialize_octal_option<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<&str> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let s = s.trim_start_matches("0o").trim_start_matches('0');
            i32::from_str_radix(s, 8)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}

/// Serialize Option<i32> as octal string
pub fn serialize_octal_option<S>(value: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&format!("0{:o}", v)),
        None => serializer.serialize_none(),
    }
}

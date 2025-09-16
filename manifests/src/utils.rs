#[cfg(feature = "readwrite")]
use color_eyre::eyre::Result;
use url::Url;

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

use std::path::Path;

use manifests::{ChallengeFlagMeta, ChallengeMetadata};
use miette::{IntoDiagnostic, Result};
use tokio::fs;

use crate::commands::apply::challenge::load_challenge_manifest;

pub async fn run(directory: &Path) -> Result<()> {
    let manifest = load_challenge_manifest::<ChallengeMetadata>(directory).await?;

    let mut readme = String::new();

    readme.push_str(&format!("# {}\n\n", &manifest.name));

    readme.push_str(&format!("***{}***\n\n", manifest.description));

    readme.push_str("### Description\n\n");

    readme.push_str(&format!("{}\n\n", &manifest.details.trim()));

    readme.push_str("### Flags\n\n");

    readme.push_str("|Slug|Type|Mount Path|Flag Content|Description|\n|---|---|---|---|---|\n");

    for flag in manifest.flags {
        // Slug
        readme.push_str(&format!("|`{}`", flag.slug));

        // Type
        readme.push_str(&format!("|{}", flag.spec.as_ref()));

        // Mount Path
        let mount_path = match &flag.spec {
            ChallengeFlagMeta::DynamicMount { mount_path, .. } => {
                format!("`{}`", mount_path.clone())
            }
            _ => "–".to_string(),
        };
        readme.push_str(&format!("|{}", mount_path));

        // Flag Content
        let flag_content = match &flag.spec {
            ChallengeFlagMeta::Static { flag } => format!("`{}`", flag.clone()),
            _ => "*Dynamic*".to_string(),
        };
        readme.push_str(&format!("|{}", flag_content));

        // Description
        readme.push_str(&format!(
            "|{}|\n",
            flag.description.unwrap_or("–".to_string()).trim()
        ));

        readme.push('\n');
    }

    readme.push_str(
        r#"
<p></p>
<p align="center">
<i>This README is generated automatically. Do not edit it directly.</i>
</p>
"#,
    );

    let readme_path = directory.join("README.md");
    fs::write(&readme_path, readme).await.into_diagnostic()?;

    Ok(())
}

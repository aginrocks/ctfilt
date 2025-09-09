use color_eyre::eyre::Result;
use headscale::apis::configuration::Configuration;
use reqwest::header::{AUTHORIZATION, HeaderMap};

use crate::settings::Settings;

pub fn init_headscale(settings: &Settings) -> Result<Configuration> {
    let headers = HeaderMap::from_iter([(
        AUTHORIZATION,
        format!("Bearer {}", settings.headscale.api_key).parse()?,
    )]);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let config = Configuration {
        base_path: settings.headscale.url.clone(),
        user_agent: Some(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )),
        client,
        ..Default::default()
    };

    Ok(config)
}

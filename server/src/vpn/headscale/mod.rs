pub mod core;
pub mod devices;

use color_eyre::Result;
pub use core::*;
pub use devices::*;
use fred::prelude::Pool;
use headscale::apis::configuration::Configuration;
use http::{HeaderMap, header::AUTHORIZATION};
use tokio::sync::broadcast::{self, Sender};

use crate::{
    settings::Settings,
    vpn::{Vpn, models::VpnEvent},
};

pub struct HeadscaleClient {
    pub config: Configuration,
    pub public_url: String,
    fred: Pool,
    pub sender: Sender<VpnEvent>,
}

impl HeadscaleClient {
    pub fn new(settings: &Settings, fred: Pool) -> Result<Self> {
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

        let (sender, _) = broadcast::channel(64);

        Ok(Self {
            config,
            public_url: settings.headscale.public_url.clone(),
            fred,
            sender,
        })
    }
}

impl Vpn for HeadscaleClient {}

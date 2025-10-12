use async_trait::async_trait;
use color_eyre::Result;
use fred::{
    clients::SubscriberClient,
    prelude::{ClientLike, EventInterface, PubsubInterface},
    types::Value,
};
use headscale::models::V1RegisterMethod;
use headscale_common::NodesChangedMessage;
use tokio::sync::broadcast::Receiver;
use tracing::info;

use crate::vpn::{
    VpnDevices,
    headscale::HeadscaleClient,
    models::{VpnDevice, VpnDeviceType, VpnEvent, VpnEventCore},
};

#[async_trait]
impl VpnDevices for HeadscaleClient {
    async fn watch(&self) -> Result<()> {
        // TODO: Set proper options
        let pubsub = SubscriberClient::new(self.fred.next().client_config(), None, None, None);
        pubsub.init().await?;

        pubsub.subscribe("vpn:nodes").await?;

        let sender = self.sender.clone();
        pubsub.on_message(move |message| {
            let sender = sender.clone();
            async move {
                info!("Received pubsub message");
                if message.channel != "vpn:nodes" {
                    return Ok(());
                }

                let Value::String(value) = message.value else {
                    return Ok(());
                };

                let value: NodesChangedMessage = serde_json::from_str(&value)?;

                let devices = value
                    .nodes
                    .into_iter()
                    .map(|node| VpnDevice {
                        r#type: match node
                            .register_method
                            .unwrap_or(V1RegisterMethod::RegisterMethodUnspecified)
                        {
                            V1RegisterMethod::RegisterMethodAuthKey => VpnDeviceType::Challenge,
                            _ => VpnDeviceType::Client,
                        },
                        name: node.name.unwrap_or_default(),
                        ip_addresses: node.ip_addresses.unwrap_or_default(),
                        last_seen: node.last_seen,
                        online: node.online.unwrap_or_default(),
                    })
                    .collect::<Vec<_>>();

                let event = VpnEvent {
                    user_subject: value.user,
                    event: VpnEventCore::StateChanged { data: devices },
                };

                sender.send(event).ok();

                Ok(())
            }
        });

        Ok(())
    }

    fn subscribe(&self) -> Receiver<VpnEvent> {
        self.sender.subscribe()
    }
}

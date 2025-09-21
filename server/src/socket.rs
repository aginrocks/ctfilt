use color_eyre::eyre::Result;
use socketioxide::{SocketIo, extract::SocketRef};
use tracing::info;

pub async fn init_io(io: SocketIo) -> Result<()> {
    io.ns("/", |s: SocketRef| {
        info!("New Socket.IO connection:",);
    });

    Ok(())
}

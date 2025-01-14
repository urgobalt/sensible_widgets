use zbus::proxy;

use crate::error::{DbusError, MainError};

use super::Network;

#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
pub trait NetworkManager {}

#[proxy(
    interface = "org.freedesktop.NetworkManager.Interface",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Interface"
)]
pub trait NetworkManagerInterface {}
pub async fn get_network_manager<'a>(
    network_manager: NetworkManagerProxy<'a>,
) -> Result<Network, MainError> {
    todo!("network_manager has not yet been implemented");
}

use std::collections::HashMap;
use std::ops::Deref;

use tokio::task::JoinSet;
use zbus::zvariant::{Dict, OwnedValue};
use zbus::{proxy, Connection};

use crate::error::{DbusError, HandleDbusError, MainError};

use self::network_manager::{get_network_manager, NetworkManagerProxy};
use self::wpa_supplicant::{get_supplicant, WpaSupplicantProxy};

mod network_manager;
mod wpa_supplicant;

#[derive(Debug)]
pub struct Network {
    ssid: String,
    signal: i16,
}

pub async fn get_network(connection: &Connection) -> Result<Network, MainError> {
    if let Ok(network_manager) = NetworkManagerProxy::builder(connection).build().await {
        let supplicant = WpaSupplicantProxy::builder(connection)
            .build()
            .await
            .handling_wpa_supplicant();
        let supplicant_network = match supplicant {
            Ok(supplicant) => get_supplicant(supplicant).await,
            Err(error) => Err(error.into()),
        };

        let nm_network = get_network_manager(network_manager).await;

        let network = match (supplicant_network, nm_network) {
            (Ok(network), Err(error)) | (Err(error), Ok(network)) => todo!(),
            (Ok(_), Ok(network)) => todo!(),
            _ => todo!(),
        };
    } else {
        Err(MainError::MissingNetworkManager)
    }
}

use std::ops::Deref;

use zbus::proxy;

use crate::error::{HandleDbusError, MainError};

use super::Network;

pub async fn get_supplicant<'a>(supplicant: WpaSupplicantProxy<'a>) -> Result<Network, MainError> {
    let objects = supplicant.interfaces().await.handling_wpa_supplicant()?;
    let mut interfaces = Vec::new();
    for obj in objects {
        let interface = WpaInterfaceProxy::builder(supplicant.connection())
            .path(obj)
            .handling_wpa_supplicant()?
            .build()
            .await
            .handling_wpa_supplicant()?;
        interfaces.push(interface);
    }

    for interface in interfaces {
        if &interface.state().await.handling_wpa_supplicant()? != "completed" {
            continue;
        }

        let obj = interface.current_bss().await.handling_wpa_supplicant()?;
        let bss = WpaBSSProxy::builder(supplicant.connection())
            .path(obj)
            .handling_wpa_supplicant()?
            .build()
            .await
            .handling_wpa_supplicant()?;

        let ssid = String::from_utf8_lossy_owned(bss.ssid().await.handling_wpa_supplicant()?);
        let signal = bss.signal().await.handling_wpa_supplicant()?;

        return Ok(Network { ssid, signal });
    }

    Err(MainError::NotConnected)
}

#[proxy(
    interface = "fi.w1.wpa_supplicant1",
    default_service = "fi.w1.wpa_supplicant1",
    default_path = "/fi/w1/wpa_supplicant1"
)]
pub trait WpaSupplicant {
    #[zbus(property)]
    fn interfaces(&self) -> zbus::Result<Vec<zbus::zvariant::ObjectPath>>;
}

impl<'a> Deref for WpaSupplicantProxy<'a> {
    type Target = zbus::Proxy<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[proxy(
    interface = "fi.w1.wpa_supplicant1.Interface",
    default_service = "fi.w1.wpa_supplicant1",
    default_path = "/fi/w1/wpa_supplicant1/Interface"
)]
pub trait WpaInterface {
    #[zbus(property, name = "CurrentBSS")]
    fn current_bss(&self) -> zbus::Result<zbus::zvariant::ObjectPath>;
    #[zbus(property)]
    fn state(&self) -> zbus::Result<String>;
}

#[proxy(
    interface = "fi.w1.wpa_supplicant1.BSS",
    default_service = "fi.w1.wpa_supplicant1",
    default_path = "/fi/w1/wpa_supplicant1/BSS"
)]
pub trait WpaBSS {
    #[zbus(property, name = "SSID")]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;
    #[zbus(property)]
    fn signal(&self) -> zbus::Result<i16>;
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<String>;
}

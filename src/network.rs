use std::i16;
use dbus::blocking::Connection;

use crate::wpa_supplicant::interface::{DisconnectReason, InterfaceState};
use crate::wpa_supplicant::WPASupplicant;
use crate::error::WPASupplicantDbusError;

pub async fn get_network() -> Result<String, WPASupplicantDbusError> {
    // let connection = Connection::new_system()?;
    // let supplicant = WPASupplicant::new(&connection);

    todo!("network has not yet been implemented");
}

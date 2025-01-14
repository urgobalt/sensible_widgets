#[derive(thiserror::Error, Debug)]
pub enum MainError {
    #[error(transparent)]
    DbusError(#[from] DbusError),
    #[error("system has no network manager")]
    MissingNetworkManager,
    #[error("no interface is connected to wifi")]
    NotConnected,
}

#[derive(thiserror::Error, Debug)]
pub enum DbusError {
    #[error("dbus error when interacting with wpa_supplicant: {0}")]
    WpaSupplicant(zbus::Error),
    #[error("dbus error when interacting with network_manager: {0}")]
    NetworkManager(zbus::Error),
    #[error("dbus error when performing vital operation: {0}")]
    System(zbus::Error),
    #[error(transparent)]
    ZVariant(#[from] zbus::zvariant::Error),
}

pub trait HandleDbusError<T> {
    fn handling_wpa_supplicant(self) -> Result<T, DbusError>;
    fn handling_network_manager(self) -> Result<T, DbusError>;
    fn handling_system(self) -> Result<T, DbusError>;
}

impl<T> HandleDbusError<T> for Result<T, zbus::Error> {
    fn handling_wpa_supplicant(self) -> Result<T, DbusError> {
        self.map_err(|err| DbusError::WpaSupplicant(err))
    }

    fn handling_network_manager(self) -> Result<T, DbusError> {
        self.map_err(|err| DbusError::NetworkManager(err))
    }

    fn handling_system(self) -> Result<T, DbusError> {
        self.map_err(|err| DbusError::System(err))
    }
}

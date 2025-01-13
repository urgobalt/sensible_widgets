use snafu::prelude::*;

#[derive(Snafu, Debug)]
pub enum SystemError {
    #[snafu(display("interfacing with wpa_supplicant: {source}"))]
    WPASupplicant { source: WPASupplicantDbusError },
}

#[derive(Snafu, Debug)]
pub struct WPASupplicantDbusError {
    source: dbus::Error,
}

use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum SystemError {
    #[snafu(display("interfacing with wpa_supplicant: {source}"))]
    WPASupplicant { source: dbus::Error },
}

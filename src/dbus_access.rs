use dbus::blocking::{Connection, Proxy};

use crate::error::SystemError;

const DBUS_TIMEOUT_MS: u64 = 5000;

pub struct Accessor<'a> {
    connection: &'a Connection,
    interface: &'a str,
    path: &'a str,
}

impl<'a> Accessor<'a> {
    pub fn new(connection: &'a Connection, interface: &'a str, path: &'a str) -> Self {
        Self {
            connection,
            interface,
            path,
        }
    }

    pub fn proxy(&self) -> Proxy<'a, &'a Connection> {
        self.connection.with_proxy(
            self.interface,
            self.path,
            std::time::Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}

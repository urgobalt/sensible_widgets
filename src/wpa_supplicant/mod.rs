use dbus::blocking::stdintf::org_freedesktop_dbus::{ObjectManager, Properties};
use dbus::blocking::{Connection, Proxy};

mod interface;

use crate::dbus_access::Accessor;
use crate::error::SystemError;

use self::interface::Interface;

const DBUS_INTERFACE: &str = "fi.w1.wpa_supplicant1";
const DBUS_PATH: &str = "/fi/w1/wpa_supplicant1";

struct WPASupplicant<'a> {
    accessor: Accessor<'a>,
}

impl<'a> WPASupplicant<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self {
            accessor: Accessor::new(connection, DBUS_INTERFACE, DBUS_PATH),
        }
    }

    pub fn proxy(&self) -> Proxy<'a, &'a Connection> {
        self.accessor.proxy()
    }

    pub fn get_interfaces(&self) -> Result<Vec<Interface>, SystemError> {
        Ok(self.proxy().get(DBUS_INTERFACE, "Interfaces")?)
    }
}

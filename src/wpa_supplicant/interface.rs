use std::convert::Infallible;
use std::str::FromStr;

use dbus::arg::{Append, Arg, ArgType, Array, Get, Iter};
use dbus::{Path, Signature};
use derive_more::derive::{From, FromStr};
use serde::Deserialize;

const DBUS_INTERFACE: &str = "fi.w1.wpa_supplicant1";

pub struct Interface<'a> {
    state: InterfaceState,
    scanning: bool,
    ap_scan: u2,
    bss_expire_age: u32,
    bss_expre_count: u32,
    country: &'a str,
    interface_name: &'a str,
    bridge_interface_name: &'a str,
    driver: &'a str,
    current_bss: Path<'a>,
    current_network: Path<'a>,
    current_auth_mode: &'a str,
    blobs: Vec<&'a str>,
    basic_service_sets: Vec<Path<'a>>,
    networks: Vec<Path<'a>>,
    fast_reauth: bool,
    scan_interval: i32,
    pkcs_11_engine_path: &'a str,
    pkcs_11_module_path: &'a str,
    disconnect_reason: DisconnectReason,
}

impl<'a> Get<'a> for Interface<'a> {
    fn get(i: &mut Iter<'a>) -> Option<Self> {
        Some(Self {
            state: i.read().ok()?,
        })
    }
}

#[derive(Debug)]
pub enum InterfaceState {
    Disconnected,
    Inactive,
    Scanning,
    Authenticating,
    Associating,
    Associated,
    QuadWayHandshake,
    GroupHandshake,
    Completed,
    Unknown,
}

impl FromStr for InterfaceState {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "disconnected" => Self::Disconnected,
            "inactive" => Self::Inactive,
            "scanning" => Self::Scanning,
            "authenticating" => Self::Authenticating,
            "associating" => Self::Associating,
            "associated" => Self::Associated,
            "4way_handshake" => Self::QuadWayHandshake,
            "groupHandshake" => Self::GroupHandshake,
            "completed" => Self::Completed,
            _ => Self::Unknown,
        })
    }
}

impl<'a> Get<'a> for InterfaceState {
    fn get(i: &mut Iter<'a>) -> Option<Self> {
        i.read::<&'a str>().ok()?.parse().ok()
    }
}

impl Arg for InterfaceState {
    const ARG_TYPE: ArgType = ArgType::String;

    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("s\0") }
    }
}

enum DisconnectReason {}

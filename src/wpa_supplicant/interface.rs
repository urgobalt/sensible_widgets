use std::convert::Infallible;
use std::str::FromStr;

use dbus::arg::{Arg, ArgType, Get, Iter};
use dbus::{Path, Signature};

const DBUS_INTERFACE: &str = "fi.w1.wpa_supplicant1";

pub struct Interface<'a> {
    state: InterfaceState,
    scanning: bool,
    ap_scan: u32,
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
    disconnect_reason: u32,
}

impl<'a> Get<'a> for Interface<'a> {
    fn get(i: &mut Iter<'a>) -> Option<Self> {
        Some(Self {
            state: i.read().ok()?,
            scanning: i.read().ok()?,
            ap_scan: i.read().ok()?,
            bss_expire_age: i.read().ok()?,
            bss_expre_count: i.read().ok()?,
            country: i.read().ok()?,
            interface_name: i.read().ok()?,
            bridge_interface_name: i.read().ok()?,
            driver: i.read().ok()?,
            current_bss: i.read().ok()?,
            current_network: i.read().ok()?,
            current_auth_mode: i.read().ok()?,
            blobs: i.read().ok()?,
            basic_service_sets: i.read().ok()?,
            networks: i.read().ok()?,
            fast_reauth: i.read().ok()?,
            scan_interval: i.read().ok()?,
            pkcs_11_engine_path: i.read().ok()?,
            pkcs_11_module_path: i.read().ok()?,
            disconnect_reason: i.read().ok()?,
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
    FourWayHandshake,
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
            "4way_handshake" => Self::FourWayHandshake,
            "group_handshake" => Self::GroupHandshake,
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

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisconnectReason {
    Reserved,
    UnspecifiedReason = 1,
    PreviousAuthNoLongerValid = 2,
    StationLeaving = 3,
    DisassociatedDueToInactivity = 4,
    APUnableToHandleAllAssociatedStations = 5,
    Class2FrameFromNonauthenticatedStation = 6,
    Class3FrameFromNonassociatedStation = 7,
    DisassociatedBecauseSendingStationLeavingBSS = 8,
    StationRequestingReassociationNotAuthenticated = 9,
    DisassociatedBecausePowerCapabilityElementIsUnacceptable = 10,
    DisassociatedBecauseSupportedChannelsElementIsUnacceptable = 11,
    InvalidInformationElement = 13,
    MessageIntegrityCodeFailure = 14,
    FourWayHandshakeTimeout = 15,
    GroupKeyHandshakeTimeout = 16,
    InformationElementInFourWayHandshakeDifferent = 17,
    InvalidGroupCipher = 18,
    InvalidPairwiseCipher = 19,
    InvalidAKMP = 20,
    UnsupportedRSNInformationElementVersion = 21,
    InvalidRSNInformationElementCapabilities = 22,
    Ieee802_1XAuthenticationFailed = 23,
    CipherSuiteRejectedBecauseOfTheSecurityPolicy = 24,
    DisassociatedForUnspecifiedQoSRelatedReason = 32,
    DisassociatedBecauseQAPLackedSufficientBandwidth = 33,
    DisassociatedBecauseOfExcessiveFrames = 34,
    DisassociatedBecauseQSTATransmittingOutsideLimitsOfTXOPs = 35,
    RequestedFromPeerQSTA = 36,
    RequestedFromPeerQSTAAsItDoesNotWantToUseTheMechanism = 37,
    RequestedFromPeerQSTAAsItReceivedFrames = 38,
    RequestedFromPeerQSTADueToTimeout = 39,
    PeerQSTA = 40,
}

impl Arg for DisconnectReason {
    const ARG_TYPE: ArgType = ArgType::UInt32;

    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("i\0") }
    }
}

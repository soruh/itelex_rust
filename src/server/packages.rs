use super::{ClientType, Package, String40Bytes};
use std::net::Ipv4Addr;

pub const LENGTH_CLIENT_UPDATE: usize = 8;
pub const LENGTH_ADDRESS_CONFIRM: usize = 4;
pub const LENGTH_END: usize = 5;
pub const LENGTH_PEER_NOT_FOUND: usize = 0;
pub const LENGTH_PEER_REPLY: usize = 100;
pub const LENGTH_FULL_QUERY: usize = 5;
pub const LENGTH_LOGIN: usize = 5;
pub const LENGTH_ACKNOWLEDGE: usize = 0;
pub const LENGTH_END_OF_LIST: usize = 0;
pub const LENGTH_PEER_SEARCH: usize = 41;

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct ClientUpdate {
    pub number: u32,
    pub pin: u16,
    pub port: u16,
}

derive_into_for_package!(ClientUpdate);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct AddressConfirm {
    pub ipaddress: Ipv4Addr,
}

derive_into_for_package!(AddressConfirm);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerQuery {
    pub number: u32,
    pub version: u8,
}

derive_into_for_package!(PeerQuery);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerNotFound {}

derive_into_for_package!(PeerNotFound);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerReply {
    pub number: u32,
    pub name: String40Bytes,
    pub flags: u16,
    pub client_type: ClientType,
    pub hostname: String40Bytes,
    pub ipaddress: Ipv4Addr,
    pub port: u16,
    pub extension: u8,
    pub pin: u16,
    pub timestamp: u32,
}

derive_into_for_package!(PeerReply);

impl PeerReply {
    pub fn extension_as_str(&self) -> Result<String, u8> {
        Ok(match self.extension {
            0 => "-".into(),
            100 => "00".into(),
            110 => "0".into(),
            x if x < 100 => format!("{:02}", x),
            x if x > 100 && x < 110 => (x - 100).to_string(),
            x => {
                // x > 110
                // entry has an invalid extension
                return Err(x);
            }
        })
    }

    pub fn disabled(&self) -> bool {
        self.flags & 2 == 0x02
    }

    pub fn flags(disabled: bool) -> u16 {
        if disabled {
            0x02
        } else {
            0x00
        }
    }

    pub fn hostname<'s>(&'s self) -> Option<&'s str> {
        if self.hostname.0.is_empty() {
            None
        } else {
            Some(&self.hostname.0)
        }
    }

    pub fn hostname_mut<'s>(&'s mut self) -> Option<&'s mut str> {
        if self.hostname.0.is_empty() {
            None
        } else {
            Some(&mut self.hostname.0)
        }
    }

    pub fn ipaddress<'s>(&'s self) -> Option<&'s Ipv4Addr> {
        if self.ipaddress.is_broadcast() {
            None
        } else {
            Some(&self.ipaddress)
        }
    }

    pub fn ipaddress_mut<'s>(&'s mut self) -> Option<&'s mut Ipv4Addr> {
        if self.ipaddress.is_broadcast() {
            None
        } else {
            Some(&mut self.ipaddress)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct FullQuery {
    pub version: u8,
    pub server_pin: u32,
}

derive_into_for_package!(FullQuery);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Login {
    pub version: u8,
    pub server_pin: u32,
}

derive_into_for_package!(Login);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Acknowledge {}

derive_into_for_package!(Acknowledge);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct EndOfList {}

derive_into_for_package!(EndOfList);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerSearch {
    pub version: u8,
    pub pattern: String40Bytes,
}

derive_into_for_package!(PeerSearch);

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Error {
    pub message: String,
}

derive_into_for_package!(Error);

impl From<String> for Error {
    fn from(string: String) -> Self {
        Error { message: string }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

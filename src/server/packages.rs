use super::{ClientType, Package, String40Bytes};
use std::net::Ipv4Addr;

pub const LENGTH_TYPE_1: usize = 8;
pub const LENGTH_TYPE_2: usize = 4;
pub const LENGTH_TYPE_3: usize = 5;
pub const LENGTH_TYPE_4: usize = 0;
pub const LENGTH_TYPE_5: usize = 100;
pub const LENGTH_TYPE_6: usize = 5;
pub const LENGTH_TYPE_7: usize = 5;
pub const LENGTH_TYPE_8: usize = 0;
pub const LENGTH_TYPE_9: usize = 0;
pub const LENGTH_TYPE_10: usize = 41;

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct ClientUpdate {
    pub number: u32,
    pub pin: u16,
    pub port: u16,
}

impl Into<Package> for ClientUpdate {
    fn into(self) -> Package {
        Package::ClientUpdate(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct AddressConfirm {
    pub ipaddress: Ipv4Addr,
}

impl Into<Package> for AddressConfirm {
    fn into(self) -> Package {
        Package::AddressConfirm(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerQuery {
    pub number: u32,
    pub version: u8,
}

impl Into<Package> for PeerQuery {
    fn into(self) -> Package {
        Package::PeerQuery(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerNotFound {}

impl Into<Package> for PeerNotFound {
    fn into(self) -> Package {
        Package::PeerNotFound(Box::new(self))
    }
}

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

impl Into<Package> for PeerReply {
    fn into(self) -> Package {
        Package::PeerReply(Box::new(self))
    }
}

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

impl Into<Package> for FullQuery {
    fn into(self) -> Package {
        Package::FullQuery(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Login {
    pub version: u8,
    pub server_pin: u32,
}

impl Into<Package> for Login {
    fn into(self) -> Package {
        Package::Login(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Acknowledge {}

impl Into<Package> for Acknowledge {
    fn into(self) -> Package {
        Package::Acknowledge(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct EndOfList {}

impl Into<Package> for EndOfList {
    fn into(self) -> Package {
        Package::EndOfList(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerSearch {
    pub version: u8,
    pub pattern: String40Bytes,
}

impl Into<Package> for PeerSearch {
    fn into(self) -> Package {
        Package::PeerSearch(Box::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Error {
    pub message: String,
}

impl Into<Package> for Error {
    fn into(self) -> Package {
        Package::Error(Box::new(self))
    }
}

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

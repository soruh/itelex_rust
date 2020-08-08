use super::{ClientType, String40Bytes};
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

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct AddressConfirm {
    pub ipaddress: Ipv4Addr,
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerQuery {
    pub number: u32,
    pub version: u8,
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerNotFound {}

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

    pub fn hostname(&self) -> Option<&str> {
        if self.hostname.0.is_empty() {
            None
        } else {
            Some(&self.hostname.0)
        }
    }

    pub fn hostname_mut(&mut self) -> Option<&mut str> {
        if self.hostname.0.is_empty() {
            None
        } else {
            Some(&mut self.hostname.0)
        }
    }

    pub fn ipaddress(&self) -> Option<&Ipv4Addr> {
        if self.ipaddress.is_broadcast() {
            None
        } else {
            Some(&self.ipaddress)
        }
    }

    pub fn ipaddress_mut(&mut self) -> Option<&mut Ipv4Addr> {
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

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Login {
    pub version: u8,
    pub server_pin: u32,
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Acknowledge {}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct EndOfList {}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerSearch {
    pub version: u8,
    pub pattern: String40Bytes,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Error {
    pub message: String,
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

impl<W: std::io::Write> binserde::Serialize<W> for Error {
    fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.message.as_bytes())?;
        writer.write_all(&[0])?;

        Ok(())
    }
}
impl<R: std::io::Read> binserde::Deserialize<R> for Error {
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        loop {
            let byte = u8::deserialize_ne(reader)?;

            if byte != 0 {
                buffer.push(byte);
            } else {
                return Ok(Error {
                    message: String::from_utf8(buffer)
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?,
                });
            }
        }
    }
}

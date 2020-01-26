use super::errors::ServerError;
use std::{
    convert::{TryFrom, TryInto},
    ffi::CString,
    io::Write,
    net::Ipv4Addr,
};

// ! This is DISGUSTING, but neccessary until we get const generics
pub(crate) struct ArrayImplWrapper<'a>(&'a [u8]);

impl<'a> TryInto<[u8; LENGTH_TYPE_5]> for ArrayImplWrapper<'a> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[u8; LENGTH_TYPE_5], Self::Error> {
        let mut res = [0_u8; LENGTH_TYPE_5];

        for (i, b) in self.0.iter().enumerate() {
            if i < LENGTH_TYPE_5 {
                res[i] = *b;
            } else {
                return Err(ServerError::ParseFailure(5).into());
            }
        }

        Ok(res)
    }
}

impl<'a> TryInto<[u8; LENGTH_TYPE_10]> for ArrayImplWrapper<'a> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[u8; LENGTH_TYPE_10], Self::Error> {
        let mut res = [0_u8; LENGTH_TYPE_10];

        for (i, b) in self.0.iter().enumerate() {
            if i < LENGTH_TYPE_10 {
                res[i] = *b;
            } else {
                return Err(ServerError::ParseFailure(10).into());
            }
        }

        Ok(res)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum ClientType {
    Deleted = 0,
    BaudotHostname = 1,
    BaudotIpaddress = 2,
    AsciiHostname = 3,
    AsciiIpaddress = 4,
    BaudotDynIp = 5,
    Email = 6,
}

impl std::fmt::Display for ClientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl TryFrom<u8> for ClientType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Deleted,
            1 => Self::BaudotHostname,
            2 => Self::BaudotIpaddress,
            3 => Self::AsciiHostname,
            4 => Self::AsciiIpaddress,
            5 => Self::BaudotDynIp,
            6 => Self::Email,

            _ => bail!("Invalid client type."),
        })
    }
}

#[cfg(feature = "serde_serialize")]
impl serde::Serialize for ClientType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[cfg(feature = "serde_deserialize")]
impl<'de> serde::Deserialize<'de> for ClientType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        ClientType::try_from(u8::deserialize(deserializer)?).map_err(|err| D::Error::custom(err))
    }
}

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

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct ClientUpdate {
    pub number: u32,
    pub pin: u16,
    pub port: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct AddressConfirm {
    pub ipaddress: Ipv4Addr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerQuery {
    pub number: u32,
    pub version: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerNotFound {}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerReply {
    pub number: u32,
    pub name: String,
    pub flags: u16,
    pub client_type: ClientType,
    pub hostname: Option<String>,
    pub ipaddress: Option<Ipv4Addr>,
    pub port: u16,
    pub extension: u8,
    pub pin: u16,
    pub timestamp: u32, // TODO
}

impl PeerReply {
    pub fn extension_as_str(&self) -> anyhow::Result<String> {
        Ok(match self.extension {
            0 => "-".into(),
            100 => "00".into(),
            110 => "0".into(),
            x if x < 100 => format!("{:02}", x),
            x if x > 100 && x < 110 => (x - 100).to_string(),
            x => {
                // x > 110
                bail!("entry has invalid extension: {}", x);
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
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct FullQuery {
    pub version: u8,
    pub server_pin: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Login {
    pub version: u8,
    pub server_pin: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Acknowledge {}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct EndOfList {}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct PeerSearch {
    pub version: u8,
    pub pattern: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Error {
    pub message: String,
}

impl TryFrom<&[u8]> for ClientUpdate {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_1 {
            bail!(ServerError::ParseFailure(1))
        }

        Ok(Self {
            number: u32::from_le_bytes(slice[0..4].try_into()?),
            pin: u16::from_le_bytes(slice[4..6].try_into()?),
            port: u16::from_le_bytes(slice[6..8].try_into()?),
        })
    }
}

impl TryFrom<&[u8]> for AddressConfirm {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_2 {
            bail!(ServerError::ParseFailure(2))
        }

        Ok(Self {
            ipaddress: {
                let array: [u8; 4] = slice[0..4].try_into()?;

                Ipv4Addr::from(array)
            },
        })
    }
}

impl TryFrom<&[u8]> for PeerQuery {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_3 {
            bail!(ServerError::ParseFailure(3))
        }

        Ok(Self {
            number: u32::from_le_bytes(slice[0..4].try_into()?),
            version: u8::from_le_bytes(slice[4..5].try_into()?),
        })
    }
}

impl TryFrom<&[u8]> for PeerNotFound {
    type Error = anyhow::Error;

    #[allow(clippy::absurd_extreme_comparisons)]
    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_4 {
            bail!(ServerError::ParseFailure(4))
        }

        Ok(Self {})
    }
}

impl TryFrom<&[u8]> for PeerReply {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_5 {
            bail!(ServerError::ParseFailure(5))
        }

        Ok(Self {
            number: u32::from_le_bytes(slice[0..4].try_into()?),
            name: string_from_slice(&slice[4..44])?,
            flags: u16::from_le_bytes(slice[44..46].try_into()?),
            client_type: ClientType::try_from(u8::from_le_bytes(slice[46..47].try_into()?))?,
            hostname: {
                let hostname = string_from_slice(&slice[47..87])?;

                if hostname.is_empty() {
                    None
                } else {
                    Some(hostname)
                }
            },
            ipaddress: {
                let octets: [u8; 4] = slice[87..91].try_into()?;

                let ipaddress = Ipv4Addr::from(octets);

                if ipaddress.is_unspecified() {
                    None
                } else {
                    Some(ipaddress)
                }
            },
            port: u16::from_le_bytes(slice[91..93].try_into()?),
            extension: u8::from_le_bytes(slice[93..94].try_into()?),
            pin: u16::from_le_bytes(slice[94..96].try_into()?),
            timestamp: u32::from_le_bytes(slice[96..100].try_into()?),
        })
    }
}

impl TryFrom<&[u8]> for FullQuery {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_6 {
            bail!(ServerError::ParseFailure(6))
        }

        Ok(Self {
            version: u8::from_le_bytes(slice[0..1].try_into()?),
            server_pin: u32::from_le_bytes(slice[1..5].try_into()?),
        })
    }
}

impl TryFrom<&[u8]> for Login {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_7 {
            bail!(ServerError::ParseFailure(7))
        }

        Ok(Self {
            version: u8::from_le_bytes(slice[0..1].try_into()?),
            server_pin: u32::from_le_bytes(slice[1..5].try_into()?),
        })
    }
}

impl TryFrom<&[u8]> for Acknowledge {
    type Error = anyhow::Error;

    #[allow(clippy::absurd_extreme_comparisons)]
    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_8 {
            bail!(ServerError::ParseFailure(8))
        }

        Ok(Self {})
    }
}

impl TryFrom<&[u8]> for EndOfList {
    type Error = anyhow::Error;

    #[allow(clippy::absurd_extreme_comparisons)]
    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_9 {
            bail!(ServerError::ParseFailure(9))
        }

        Ok(Self {})
    }
}

impl TryFrom<&[u8]> for PeerSearch {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < LENGTH_TYPE_10 {
            bail!(ServerError::ParseFailure(10))
        }

        Ok(Self {
            version: u8::from_le_bytes(slice[0..1].try_into()?),
            pattern: string_from_slice(slice[1..41].try_into()?)?,
        })
    }
}

impl TryFrom<&[u8]> for Error {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            message: string_from_slice(slice)?,
        })
    }
}

impl TryInto<Vec<u8>> for ClientUpdate {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::with_capacity(LENGTH_TYPE_1);

        res.write_all(&self.number.to_le_bytes())?;
        res.write_all(&self.pin.to_le_bytes())?;
        res.write_all(&self.port.to_le_bytes())?;

        Ok(res)
    }
}

impl TryInto<Vec<u8>> for AddressConfirm {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::with_capacity(LENGTH_TYPE_2);

        res.write_all(&self.ipaddress.octets())?;

        Ok(res)
    }
}

impl TryInto<Vec<u8>> for PeerQuery {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::with_capacity(LENGTH_TYPE_3);

        res.write_all(&self.number.to_le_bytes())?;

        res.write_all(&self.version.to_le_bytes())?;

        Ok(res)
    }
}

impl TryInto<Vec<u8>> for PeerNotFound {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl TryInto<Vec<u8>> for PeerReply {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::with_capacity(LENGTH_TYPE_5);

        res.write_all(&self.number.to_le_bytes())?;
        res.write_all(&array_from_string(self.name))?;
        res.write_all(&self.flags.to_le_bytes())?;
        res.write_all(&(self.client_type as u8).to_le_bytes())?;
        res.write_all(&array_from_string(self.hostname.unwrap_or_default()))?;
        res.write_all(&self.ipaddress.map(|e| e.octets()).unwrap_or([0, 0, 0, 0]))?;
        res.write_all(&self.port.to_le_bytes())?;
        res.write_all(&self.extension.to_le_bytes())?;
        res.write_all(&self.pin.to_le_bytes())?;
        res.write_all(&self.timestamp.to_le_bytes())?;

        Ok(res)
    }
}

impl TryInto<Vec<u8>> for FullQuery {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::with_capacity(LENGTH_TYPE_6);

        res.write_all(&self.version.to_le_bytes())?;

        res.write_all(&self.server_pin.to_le_bytes())?;

        Ok(res)
    }
}

impl TryInto<Vec<u8>> for Login {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::with_capacity(LENGTH_TYPE_7);

        res.write_all(&self.version.to_le_bytes())?;

        res.write_all(&self.server_pin.to_le_bytes())?;

        Ok(res)
    }
}

impl TryInto<Vec<u8>> for Acknowledge {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl TryInto<Vec<u8>> for EndOfList {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl TryInto<Vec<u8>> for PeerSearch {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::with_capacity(LENGTH_TYPE_10);

        res.write_all(&self.version.to_le_bytes())?;

        res.write_all(&array_from_string(self.pattern))?;

        Ok(res)
    }
}

impl TryInto<Vec<u8>> for Error {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        let mut res: Vec<u8> = CString::new(self.message)?.try_into()?;

        res.push(0);

        if res.len() > 0xff {
            bail!(ServerError::SerializeFailure(255));
        }

        Ok(res)
    }
}

// TODO: Box some of the contents, so that not all instances
// TODO: of this enum are >= 101 Bytes
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub enum Package {
    ClientUpdate(ClientUpdate),
    AddressConfirm(AddressConfirm),
    PeerQuery(PeerQuery),
    PeerNotFound(PeerNotFound),
    PeerReply(PeerReply),
    FullQuery(FullQuery),
    Login(Login),
    Acknowledge(Acknowledge),
    EndOfList(EndOfList),
    PeerSearch(PeerSearch),
    Error(Error),
}

impl Package {
    pub fn parse(package_type: u8, slice: &[u8]) -> anyhow::Result<Self> {
        Ok(match package_type {
            1 => Self::ClientUpdate(ClientUpdate::try_from(slice)?),
            2 => Self::AddressConfirm(AddressConfirm::try_from(slice)?),
            3 => Self::PeerQuery(PeerQuery::try_from(slice)?),
            4 => Self::PeerNotFound(PeerNotFound::try_from(slice)?),
            5 => Self::PeerReply(PeerReply::try_from(slice)?),
            6 => Self::FullQuery(FullQuery::try_from(slice)?),
            7 => Self::Login(Login::try_from(slice)?),
            8 => Self::Acknowledge(Acknowledge::try_from(slice)?),
            9 => Self::EndOfList(EndOfList::try_from(slice)?),
            10 => Self::PeerSearch(PeerSearch::try_from(slice)?),
            255 => Self::Error(Error::try_from(slice)?),

            _ => bail!(ServerError::ParseFailure(package_type)),
        })
    }

    pub fn package_type(&self) -> u8 {
        match self {
            Self::ClientUpdate(_) => 1,
            Self::AddressConfirm(_) => 2,
            Self::PeerQuery(_) => 3,
            Self::PeerNotFound(_) => 4,
            Self::PeerReply(_) => 5,
            Self::FullQuery(_) => 6,
            Self::Login(_) => 7,
            Self::Acknowledge(_) => 8,
            Self::EndOfList(_) => 9,
            Self::PeerSearch(_) => 10,
            Self::Error(_) => 255,
        }
    }
}

impl TryInto<Vec<u8>> for Package {
    type Error = anyhow::Error;

    fn try_into(self: Self) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::ClientUpdate(pkg) => pkg.try_into(),
            Self::AddressConfirm(pkg) => pkg.try_into(),
            Self::PeerQuery(pkg) => pkg.try_into(),
            Self::PeerNotFound(pkg) => pkg.try_into(),
            Self::PeerReply(pkg) => pkg.try_into(),
            Self::FullQuery(pkg) => pkg.try_into(),
            Self::Login(pkg) => pkg.try_into(),
            Self::Acknowledge(pkg) => pkg.try_into(),
            Self::EndOfList(pkg) => pkg.try_into(),
            Self::PeerSearch(pkg) => pkg.try_into(),
            Self::Error(pkg) => pkg.try_into(),
        }
    }
}

fn array_from_string(mut input: String) -> [u8; 40] {
    let mut buf: [u8; 40] = [0; 40];

    input.truncate(39); // ensure we don't write over capaciry and leave one 0 byte at the end

    for (i, b) in input.into_bytes().into_iter().enumerate() {
        buf[i] = b;
    }

    buf
}

fn string_from_slice(slice: &[u8]) -> anyhow::Result<String> {
    let mut end_of_content = slice.len();

    for (i, val) in slice.iter().enumerate() {
        if *val == 0 {
            end_of_content = i;

            break;
        }
    }

    Ok(CString::new(&slice[0..end_of_content])?.into_string()?)
}

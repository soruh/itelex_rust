use binserde::{Deserialize, Serialize};
use std::{convert::TryFrom, ffi::CString, net::Ipv4Addr};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ClientType {
    Deleted = 0,
    BaudotHostname = 1,
    BaudotIpaddress = 2,
    AsciiHostname = 3,
    AsciiIpaddress = 4,
    BaudotDynIp = 5,
    Email = 6,
}

impl<W> binserde::Serialize<W> for ClientType
where
    W: std::io::Write,
{
    fn serialize_ne(&self, _: &mut W) -> std::io::Result<()> {
        panic!("I-Telex packages are always in Little Endian format")
    }
    fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
        (*self as u8).serialize_le(writer)
    }
}

impl<R> binserde::Deserialize<R> for ClientType
where
    R: std::io::Read,
{
    fn deserialize_ne(_: &mut R) -> std::io::Result<Self> {
        panic!("I-Telex packages are always in Little Endian format")
    }
    fn deserialize_le(reader: &mut R) -> std::io::Result<Self> {
        ClientType::try_from(u8::deserialize_le(reader)?)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }
}

impl std::fmt::Display for ClientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl TryFrom<u8> for ClientType {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Deleted,
            1 => Self::BaudotHostname,
            2 => Self::BaudotIpaddress,
            3 => Self::AsciiHostname,
            4 => Self::AsciiIpaddress,
            5 => Self::BaudotDynIp,
            6 => Self::Email,

            _ => return Err(std::io::ErrorKind::InvalidInput.into()),
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

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct String40Bytes(pub String);
impl From<String> for String40Bytes {
    fn from(string: String) -> Self {
        String40Bytes(string)
    }
}

impl<W> binserde::Serialize<W> for String40Bytes
where
    W: std::io::Write,
{
    fn serialize_ne(&self, writer: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut string = self.0.clone().into_bytes();

        string.truncate(39); // remove all content that will not fit into the buffer
        string.resize_with(40, || 0); // extend the string to fit the buffer, padding with zeros
        writer.write_all(&string)?; // write the string to the buffer

        Ok(())
    }
}

impl<R> binserde::Deserialize<R> for String40Bytes
where
    R: std::io::Read,
{
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = [0u8; 40];

        reader.read_exact(&mut buffer)?;

        let end_of_content = buffer
            .iter()
            .position(|x| *x == 0)
            .unwrap_or_else(|| buffer.len());

        let string = CString::new(&buffer[0..end_of_content])?
            .into_string()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

        Ok(Self(string))
    }
}

impl std::ops::Deref for String40Bytes {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for String40Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Default for String40Bytes {
    fn default() -> Self {
        Self(String::new())
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

    pub fn package_length(&self) -> u8 {
        let res = match self {
            Self::ClientUpdate(_) => LENGTH_TYPE_1,
            Self::AddressConfirm(_) => LENGTH_TYPE_2,
            Self::PeerQuery(_) => LENGTH_TYPE_3,
            Self::PeerNotFound(_) => LENGTH_TYPE_4,
            Self::PeerReply(_) => LENGTH_TYPE_5,
            Self::FullQuery(_) => LENGTH_TYPE_6,
            Self::Login(_) => LENGTH_TYPE_7,
            Self::Acknowledge(_) => LENGTH_TYPE_8,
            Self::EndOfList(_) => LENGTH_TYPE_9,
            Self::PeerSearch(_) => LENGTH_TYPE_10,
            Self::Error(val) => string_byte_length(&val.message),
        };

        res as u8
    }

    pub fn serialize_header(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        // Note: native endianess is always correct here, since an u8 is only one byte...
        let package_type = self.package_type();
        let package_length = self.package_length();

        package_type.serialize_ne(writer)?;
        package_length.serialize_ne(writer)?;

        Ok(())
    }

    pub fn deserialize_header(reader: &mut impl std::io::Read) -> std::io::Result<(u8, u8)> {
        // Note: native endianess is always correct here, since an u8 is only one byte...
        let package_type = u8::deserialize_ne(reader)?;
        let package_length = u8::deserialize_ne(reader)?;

        Ok((package_type, package_length))
    }
}

impl<W> binserde::Serialize<W> for Package
where
    W: std::io::Write,
{
    fn serialize_ne(&self, _: &mut W) -> std::io::Result<()> {
        panic!("I-Telex packages are always in Little Endian format")
    }
    fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
        self.serialize_header(writer)?;

        match self {
            Self::ClientUpdate(pkg) => (*pkg).serialize_le(writer),
            Self::AddressConfirm(pkg) => (*pkg).serialize_le(writer),
            Self::PeerQuery(pkg) => (*pkg).serialize_le(writer),
            Self::PeerNotFound(pkg) => (*pkg).serialize_le(writer),
            Self::PeerReply(pkg) => (*pkg).serialize_le(writer),
            Self::FullQuery(pkg) => (*pkg).serialize_le(writer),
            Self::Login(pkg) => (*pkg).serialize_le(writer),
            Self::Acknowledge(pkg) => (*pkg).serialize_le(writer),
            Self::EndOfList(pkg) => (*pkg).serialize_le(writer),
            Self::PeerSearch(pkg) => (*pkg).serialize_le(writer),
            Self::Error(pkg) => serialize_string(&pkg.message, writer),
        }
    }
}

impl<R> binserde::Deserialize<R> for Package
where
    R: std::io::Read,
{
    fn deserialize_ne(_: &mut R) -> std::io::Result<Self> {
        panic!("I-Telex packages are always in Little Endian format")
    }

    fn deserialize_le(reader: &mut R) -> std::io::Result<Self> {
        let (package_type, package_length) = Package::deserialize_header(reader)?;
        let mut buffer = vec![0u8; package_length as usize];
        reader.read_exact(&mut buffer)?;
        let mut buffer = std::io::Cursor::new(buffer);
        Ok(match package_type {
            1 => Self::ClientUpdate(ClientUpdate::deserialize_le(&mut buffer)?),
            2 => Self::AddressConfirm(AddressConfirm::deserialize_le(&mut buffer)?),
            3 => Self::PeerQuery(PeerQuery::deserialize_le(&mut buffer)?),
            4 => Self::PeerNotFound(PeerNotFound::deserialize_le(&mut buffer)?),
            5 => Self::PeerReply(PeerReply::deserialize_le(&mut buffer)?),
            6 => Self::FullQuery(FullQuery::deserialize_le(&mut buffer)?),
            7 => Self::Login(Login::deserialize_le(&mut buffer)?),
            8 => Self::Acknowledge(Acknowledge::deserialize_le(&mut buffer)?),
            9 => Self::EndOfList(EndOfList::deserialize_le(&mut buffer)?),
            10 => Self::PeerSearch(PeerSearch::deserialize_le(&mut buffer)?),
            255 => Self::Error(Error {
                message: deserialize_string(buffer.into_inner())?,
            }),

            _ => Err(std::io::ErrorKind::InvalidData)?,
        })
    }
}

fn string_byte_length(string: &str) -> usize {
    (string.bytes().count() + 1).min(0xff)
}

fn serialize_string(string: &str, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    let bytes: Vec<u8> = string.bytes().take(255).collect();
    writer.write_all(&bytes)?;
    writer.write_all(&[0u8])
}

fn deserialize_string(buffer: Vec<u8>) -> std::io::Result<String> {
    let end_of_content = buffer
        .iter()
        .position(|x| *x == 0)
        .unwrap_or_else(|| buffer.len());

    let string = CString::new(&buffer[0..end_of_content])?
        .into_string()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use binserde::{Deserialize, Serialize};
    use std::io::Cursor;
    use std::net::Ipv4Addr;

    fn test_all(package: Package, serialized: Vec<u8>) {
        {
            let mut cursor = Cursor::new(serialized.clone());
            assert_eq!(
                Package::deserialize_le(&mut cursor).expect("Package::deserialize_le failed"),
                package,
                "deserialize_le created unexpected result"
            );

            let mut res = Vec::with_capacity(serialized.len());

            package
                .serialize_le(&mut res)
                .expect("package.serialize_le failed");

            assert_eq!(res, serialized, "serialize_le created unexpected result");
        }
    }

    #[test]

    fn type_1() {
        let serialized: Vec<u8> = vec![
            // header:
            1, 8, // number:
            0x0f, 0xf0, 0x00, 0xff, // pin:
            0x0f, 0xf0, // port:
            0xf0, 0x0f,
        ];

        let package = Package::ClientUpdate(ClientUpdate {
            number: 0xff_00_f0_0f,
            pin: 0xf0_0f,
            port: 0x0f_f0,
        });

        test_all(package, serialized);
    }

    #[test]

    fn type_2() {
        let serialized: Vec<u8> = vec![
            // header:
            2, 4, // ipaddress
            0xff, 0x00, 0xf0, 0x0f,
        ];

        let package = Package::AddressConfirm(AddressConfirm {
            ipaddress: Ipv4Addr::from([0xff, 0x00, 0xf0, 0x0f]),
        });

        test_all(package, serialized);
    }

    #[test]

    fn type_3() {
        let serialized: Vec<u8> = vec![
            // header:
            3, 5, // number:
            0x44, 0x33, 0x22, 0x11, // version:
            0xf7,
        ];

        let package = Package::PeerQuery(PeerQuery {
            number: 0x11_22_33_44,
            version: 0xf7,
        });

        test_all(package, serialized);
    }

    #[test]

    fn type_4() {
        let serialized: Vec<u8> = vec![4, 0];

        let package = Package::PeerNotFound(PeerNotFound {});

        test_all(package, serialized);
    }

    #[test]

    fn type_5() {
        let serialized: Vec<u8> = vec![
            // header:
            5, 100, // number:
            4, 3, 2, 1, // name:
            84, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // flags:
            2, 0, // client_type:
            5, // hostname:
            104, 111, 115, 116, 46, 110, 97, 109, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // ipaddress:
            8, 9, 0x0a, 0x0b, // port:
            0x0d, 0x0c, // extension:
            0x0e, // pin:
            0x10, 0x0f, //timestamp:
            0x14, 0x13, 0x12, 0x11,
        ];

        let package = Package::PeerReply(PeerReply {
            number: 0x01_02_03_04,
            name: String::from("Test").into(),
            flags: PeerReply::flags(true),
            client_type: ClientType::BaudotDynIp,
            hostname: String::from("host.name").into(),
            ipaddress: Ipv4Addr::from(0x08_09_0a_0b),
            port: 0x0c_0d,
            extension: 0x0e,
            pin: 0x0f_10,
            timestamp: 0x11_12_13_14,
        });

        test_all(package, serialized);
    }

    #[test]

    fn type_6() {
        let serialized: Vec<u8> = vec![6, 5, 0x0f, 0x11, 0x22, 0x33, 0x44];

        let package = Package::FullQuery(FullQuery {
            server_pin: 0x44_33_22_11,
            version: 0x0f,
        });

        test_all(package, serialized);
    }

    #[test]

    fn type_7() {
        let serialized: Vec<u8> = vec![7, 5, 0x0f, 0x11, 0x22, 0x33, 0x44];

        let package = Package::Login(Login {
            server_pin: 0x44_33_22_11,
            version: 0x0f,
        });

        test_all(package, serialized);
    }

    #[test]

    fn type_8() {
        let serialized: Vec<u8> = vec![8, 0];

        let package = Package::Acknowledge(Acknowledge {});

        test_all(package, serialized);
    }

    #[test]

    fn type_9() {
        let serialized: Vec<u8> = vec![9, 0];

        let package = Package::EndOfList(EndOfList {});

        test_all(package, serialized);
    }

    #[test]

    fn type_10() {
        let serialized: Vec<u8> = vec![
            // header:
            10, 41,  // version:
            240, // pattern:
            80, 97, 116, 116, 101, 114, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let package = Package::PeerSearch(PeerSearch {
            pattern: String::from("Pattern").into(),
            version: 0xf0,
        });

        test_all(package, serialized);
    }

    #[test]

    fn type_255() {
        let serialized: Vec<u8> = vec![
            // header:
            0xff, 22, // message:
            65, 110, 32, 69, 114, 114, 111, 114, 32, 104, 97, 115, 32, 111, 99, 99, 117, 114, 101,
            100, 33, 0,
        ];

        let package = Package::Error(Error {
            message: String::from("An Error has occured!"),
        });

        test_all(package, serialized);
    }
}

use super::{packages::*, Deserialize, Serialize};
use crate::{deserialize_string, serialize_string, string_byte_length};

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub enum Package {
    ClientUpdate(Box<ClientUpdate>),
    AddressConfirm(Box<AddressConfirm>),
    PeerQuery(Box<PeerQuery>),
    PeerNotFound(Box<PeerNotFound>),
    PeerReply(Box<PeerReply>),
    FullQuery(Box<FullQuery>),
    Login(Box<Login>),
    Acknowledge(Box<Acknowledge>),
    EndOfList(Box<EndOfList>),
    PeerSearch(Box<PeerSearch>),
    Error(Box<Error>),
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
        (match self {
            Self::ClientUpdate(_) => LENGTH_CLIENT_UPDATE,
            Self::AddressConfirm(_) => LENGTH_ADDRESS_CONFIRM,
            Self::PeerQuery(_) => LENGTH_END,
            Self::PeerNotFound(_) => LENGTH_PEER_NOT_FOUND,
            Self::PeerReply(_) => LENGTH_PEER_REPLY,
            Self::FullQuery(_) => LENGTH_FULL_QUERY,
            Self::Login(_) => LENGTH_LOGIN,
            Self::Acknowledge(_) => LENGTH_ACKNOWLEDGE,
            Self::EndOfList(_) => LENGTH_END_OF_LIST,
            Self::PeerSearch(_) => LENGTH_PEER_SEARCH,
            Self::Error(val) => string_byte_length(&val.message),
        }) as u8
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
            Self::ClientUpdate(pkg) => pkg.serialize_le(writer),
            Self::AddressConfirm(pkg) => pkg.serialize_le(writer),
            Self::PeerQuery(pkg) => pkg.serialize_le(writer),
            Self::PeerNotFound(pkg) => pkg.serialize_le(writer),
            Self::PeerReply(pkg) => pkg.serialize_le(writer),
            Self::FullQuery(pkg) => pkg.serialize_le(writer),
            Self::Login(pkg) => pkg.serialize_le(writer),
            Self::Acknowledge(pkg) => pkg.serialize_le(writer),
            Self::EndOfList(pkg) => pkg.serialize_le(writer),
            Self::PeerSearch(pkg) => pkg.serialize_le(writer),
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
            1 => ClientUpdate::deserialize_le(&mut buffer)?.into(),
            2 => AddressConfirm::deserialize_le(&mut buffer)?.into(),
            3 => PeerQuery::deserialize_le(&mut buffer)?.into(),
            4 => PeerNotFound::deserialize_le(&mut buffer)?.into(),
            5 => PeerReply::deserialize_le(&mut buffer)?.into(),
            6 => FullQuery::deserialize_le(&mut buffer)?.into(),
            7 => Login::deserialize_le(&mut buffer)?.into(),
            8 => Acknowledge::deserialize_le(&mut buffer)?.into(),
            9 => EndOfList::deserialize_le(&mut buffer)?.into(),
            10 => PeerSearch::deserialize_le(&mut buffer)?.into(),
            255 => Error::from(deserialize_string(buffer.into_inner())?).into(),

            _ => Err(std::io::ErrorKind::InvalidData)?,
        })
    }
}

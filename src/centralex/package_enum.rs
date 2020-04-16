use super::*;
use crate::{deserialize_string, serialize_string, string_byte_length};

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub enum Package {
    RemConnect(RemConnect),
    RemConfirm(RemConfirm),
    RemCall(RemCall),
    RemAck(RemAck),
    End(End),
    Reject(Reject),
}

impl Package {
    pub fn package_type(&self) -> u8 {
        match self {
            Self::RemConnect(_) => 0x81,
            Self::RemConfirm(_) => 0x82,
            Self::RemCall(_) => 0x83,
            Self::RemAck(_) => 0x84,
            Self::End(_) => 3,
            Self::Reject(_) => 4,
        }
    }

    pub fn package_length(&self) -> u8 {
        (match self {
            Self::RemConnect(_) => LENGTH_REM_CONNECT,
            Self::RemConfirm(_) => LENGTH_REM_CONFIRM,
            Self::RemCall(_) => LENGTH_REM_CALL,
            Self::RemAck(_) => LENGTH_REM_ACK,
            Self::End(_) => LENGTH_END,
            Self::Reject(pkg) => string_byte_length(&pkg.message),
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
            Self::RemConnect(pkg) => pkg.serialize_le(writer),
            Self::RemConfirm(pkg) => pkg.serialize_le(writer),
            Self::RemCall(pkg) => pkg.serialize_le(writer),
            Self::RemAck(pkg) => pkg.serialize_le(writer),
            Self::End(pkg) => pkg.serialize_le(writer),
            Self::Reject(pkg) => serialize_string(&pkg.message, writer),
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
            129 => RemConnect::deserialize_le(&mut buffer)?.into(),
            130 => RemConfirm::deserialize_le(&mut buffer)?.into(),
            131 => RemCall::deserialize_le(&mut buffer)?.into(),
            132 => RemAck::deserialize_le(&mut buffer)?.into(),
            3 => End::deserialize_le(&mut buffer)?.into(),
            4 => Reject::from(deserialize_string(buffer.into_inner())?).into(),

            _ => Err(std::io::ErrorKind::InvalidData)?,
        })
    }
}

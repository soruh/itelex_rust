use crate::{deserialize_string, serialize_string, string_byte_length};
use binserde::{Deserialize, Serialize};

pub const LENGTH_END: usize = 0;

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct End {}

derive_into_for_package!(End);

pub const LENGTH_HEARTBEAT: usize = 0;

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Heartbeat {}

derive_into_for_package!(Heartbeat);

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Reject {
    pub message: String,
}

impl<W: std::io::Write> binserde::Serialize<W> for Reject {
    fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.message.as_bytes())?;
        writer.write_all(&[0])?;

        Ok(())
    }
}
impl<R: std::io::Read> binserde::Deserialize<R> for Reject {
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        loop {
            let byte = u8::deserialize_ne(reader)?;

            if byte != 0 {
                buffer.push(byte);
            } else {
                return Ok(Reject {
                    message: String::from_utf8(buffer)
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?,
                });
            }
        }
    }
}

impl From<String> for Reject {
    fn from(string: String) -> Self {
        Reject { message: string }
    }
}

impl std::fmt::Display for Reject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Reject {}

derive_into_for_package!(Reject);

#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
#[non_exhaustive] // TODO: remove once complete
pub enum Package {
    End(End),
    Reject(Reject),
    Heartbeat(Heartbeat),
    // TODO
}

impl Package {
    pub fn package_type(&self) -> u8 {
        match self {
            Self::Heartbeat(_) => 0,
            Self::End(_) => 3,
            Self::Reject(_) => 4,
        }
    }

    pub fn package_length(&self) -> u8 {
        (match self {
            Self::Heartbeat(_) => LENGTH_HEARTBEAT,
            Self::End(_) => LENGTH_END,
            Self::Reject(val) => string_byte_length(&val.message),
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
            Self::Heartbeat(pkg) => pkg.serialize_le(writer),
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
            0 => Heartbeat::deserialize_le(&mut buffer)?.into(),
            3 => End::deserialize_le(&mut buffer)?.into(),
            4 => Reject::from(deserialize_string(buffer.into_inner())?).into(),

            _ => Err(std::io::ErrorKind::InvalidData)?,
        })
    }
}

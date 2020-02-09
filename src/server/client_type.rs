use std::convert::TryFrom;

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

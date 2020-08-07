#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemConnect {
    pub number: u32,
    pub pin: u16,
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemConfirm {}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemCall {
    pub remote_ip_v4: std::net::Ipv4Addr,
    pub remote_ip_v6: std::net::Ipv6Addr,
}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemAck {}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct End {}

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct Heartbeat {}

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

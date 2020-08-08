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

#[non_exhaustive] // TODO: remove once complete
package_class! {Client("Client"),
    Heartbeat = 0x00,
    End = 0x03,
    Reject = 0x04,
    // TODO
}

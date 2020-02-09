#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct String40Bytes(pub String);
impl From<String> for String40Bytes {
    fn from(string: String) -> Self {
        String40Bytes(string)
    }
}

impl From<&str> for String40Bytes {
    fn from(string: &str) -> Self {
        String40Bytes(String::from(string))
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

        let string = String::from_utf8_lossy(&buffer[0..end_of_content]).into();

        Ok(Self(string))
    }
}

impl std::fmt::Display for String40Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
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

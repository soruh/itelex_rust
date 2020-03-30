macro_rules! derive_into_for_package {
    ($package_name: ident) => {
        impl Into<Package> for $package_name {
            fn into(self) -> Package {
                Package::$package_name(Box::new(self))
            }
        }
    };
}


pub fn string_byte_length(string: &str) -> usize {
    (string.bytes().count() + 1).min(0xff)
}

pub fn serialize_string(string: &str, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    let bytes: Vec<u8> = string.bytes().take(255).collect();
    writer.write_all(&bytes)?;
    writer.write_all(&[0u8])
}

pub fn deserialize_string(buffer: Vec<u8>) -> std::io::Result<String> {
    let end_of_content = buffer
        .iter()
        .position(|x| *x == 0)
        .unwrap_or_else(|| buffer.len());

    let string = String::from_utf8_lossy(&buffer[0..end_of_content]).into();

    Ok(string)
}


#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "centralex")]
pub mod centralex;

pub use binserde::{Deserialize, Serialize};

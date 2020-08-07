use core::any::Any;

#[derive(Debug, Copy, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
struct Header {
    package_type: u8,
    package_length: u8,
}

// This trait is not object safe! Use Any
pub trait PackageTrait
where
    Self: Sized,
{
    type Class;
    fn to_package(self) -> Package<Self::Class>
    where
        Self: 'static,
    {
        Package::new(self)
    }
    fn package_type(&self) -> Self::Class;
    fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>;
    fn deserialize(reader: &mut impl std::io::Read) -> std::io::Result<Self>;
}
pub struct Package<T>(Box<dyn Any>, std::marker::PhantomData<T>);

impl<T> Package<T> {
    fn new<P: PackageTrait + 'static>(pkg: P) -> Self {
        Package(Box::new(pkg), Default::default())
    }
}

impl<T> std::ops::Deref for Package<T> {
    type Target = Box<dyn Any>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Package<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug)]
struct NotAPackage;
impl std::error::Error for NotAPackage {}
impl std::fmt::Display for NotAPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! package {
    ($class: ident, $($package_name: ident = $discriminant: literal,)*) => {
        use crate::{Package, PackageTrait, Header, NotAPackage};
        use binserde::{Deserialize, Serialize};

        use std::convert::{TryInto, TryFrom};

        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum $class {
            $($package_name = $discriminant,)*
        }


        impl Package<$class> {
            pub fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
                $(
                    if let Some(pkg) = self.0.downcast_ref::<$package_name>() {
                        let mut buffer = Vec::new();
                        pkg.serialize(&mut buffer)?;

                        Header {
                            package_type: pkg.package_type() as u8,
                            package_length: buffer.len() as u8,
                        }.serialize_ne(writer)?;

                        writer.write_all(&buffer)?;

                        return Ok(());
                    }
                )*

                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, NotAPackage));
            }
            pub fn deserialize(reader: &mut impl std::io::Read) -> std::io::Result<Self> {
                let header = Header::deserialize_ne(reader)?;
                let mut buffer = vec![0; header.package_length as usize];

                reader.read_exact(&mut buffer)?;

                match header.package_type.try_into().map_err(|_err| std::io::Error::new(std::io::ErrorKind::InvalidInput, NotAPackage))? {
                    $($class::$package_name => {
                        $package_name::deserialize(&mut std::io::Cursor::new(buffer)).map(Package::new)
                    })*
                }


            }
        }

        impl Into<u8> for $class {
            fn into(self) -> u8 {
                self as u8
            }
        }

        impl TryFrom<u8> for $class {
            type Error = ();
            fn try_from(discriminant: u8) -> Result<Self, Self::Error> {
                match discriminant {
                    $($discriminant => Ok($class::$package_name),)*
                    _ => Err(())
                }
            }
        }

        $(
            impl PackageTrait for $package_name {
                type Class = $class;
                fn package_type(&self) -> Self::Class {
                    $class::$package_name
                }
                fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
                    use binserde::Serialize;

                    self.serialize_le(writer)
                }
                fn deserialize(reader: &mut impl std::io::Read) -> std::io::Result<Self> {
                    use binserde::Deserialize;

                    $package_name::deserialize_le(reader)
                }
            }
        )*
    };
}

macro_rules! derive_into_for_package {
    ($package_name: ident) => {
        impl Into<Package> for $package_name {
            fn into(self) -> Package {
                Package::$package_name(self)
            }
        }
    };
}

pub(crate) fn string_byte_length(string: &str) -> usize {
    (string.bytes().count() + 1).min(0xff)
}

pub(crate) fn serialize_string(
    string: &str,
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    let bytes: Vec<u8> = string.bytes().take(255).collect();
    writer.write_all(&bytes)?;
    writer.write_all(&[0u8])
}

pub(crate) fn deserialize_string(buffer: Vec<u8>) -> std::io::Result<String> {
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

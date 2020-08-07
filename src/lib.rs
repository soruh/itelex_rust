use core::any::Any;

pub trait Class {}

#[derive(Debug, Copy, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
struct Header {
    package_type: u8,
    package_length: u8,
}

#[cfg(all(feature = "serde_serialize", feature = "serde_deserialize"))]
pub trait SerdeBounds: serde::Serialize + serde::Deserialize<'static> {}
#[cfg(all(feature = "serde_serialize", not(feature = "serde_deserialize")))]
pub trait SerdeBounds: serde::Serialize {}
#[cfg(all(not(feature = "serde_serialize"), feature = "serde_deserialize"))]
pub trait SerdeBounds: serde::Deserialize<'static> {}
#[cfg(not(any(feature = "serde_serialize", feature = "serde_deserialize")))]
pub trait SerdeBounds {}

impl<T: PackageBody> SerdeBounds for T {}

pub trait PackageBody
where
    Self: Sized
        + std::fmt::Debug
        + std::cmp::Eq
        + std::cmp::PartialEq
        + Clone
        + 'static
        + SerdeBounds,
{
    type Class: Class;
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

impl<T: Class> Package<T> {
    pub fn new<P: PackageBody<Class = T> + 'static>(pkg: P) -> Self {
        Package(Box::new(pkg), Default::default())
    }
    pub fn downcast_ref<P: PackageBody + Any>(&self) -> Option<&P> {
        self.0.downcast_ref::<P>()
    }
    pub fn downcast_mut<P: PackageBody + Any>(&mut self) -> Option<&mut P> {
        self.0.downcast_mut::<P>()
    }
    pub fn is<P: PackageBody + Any>(&self) -> bool {
        self.0.is::<P>()
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

macro_rules! package_class {
    ($class: ident, $($package_name: ident = $discriminant: literal,)*) => {
        use crate::{Package, PackageBody, Header, NotAPackage, Class};
        use binserde::{Deserialize, Serialize};

        use std::convert::{TryInto, TryFrom};

        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum $class {
            $($package_name = $discriminant,)*
        }

        impl Class for $class {}

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
            fn try_from(discriminant: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
                match discriminant {
                    $($discriminant => Ok($class::$package_name),)*
                    _ => Err(())
                }
            }
        }

        $(
            impl PackageBody for $package_name {
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

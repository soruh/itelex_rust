package_class! {
    Server("Server"),
    ClientUpdate = 0x01,
    AddressConfirm = 0x02,
    PeerQuery = 0x03,
    PeerNotFound = 0x04,
    PeerReply = 0x05,
    FullQuery = 0x06,
    Login = 0x07,
    Acknowledge = 0x08,
    EndOfList = 0x09,
    PeerSearch = 0x0A,
    Error = 0xFF,
}

#[cfg(test)]
mod tests;

mod client_type;
pub use client_type::*;

mod string_40_bytes;
pub use string_40_bytes::*;

mod packages;
pub use packages::*;

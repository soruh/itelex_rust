use super::*;

derive_into_for_package!(End);
derive_into_for_package!(Reject);

pub const LENGTH_REM_CONNECT: usize = 6;
pub const LENGTH_REM_CONFIRM: usize = 0;
pub const LENGTH_REM_CALL: usize = 0;
pub const LENGTH_REM_ACK: usize = 0;

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemConnect {
    pub number: u32,
    pub pin: u16,
}

derive_into_for_package!(RemConnect);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemConfirm {}

derive_into_for_package!(RemConfirm);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemCall {
    pub remote_ip_v4: std::net::Ipv4Addr,
    pub remote_ip_v6: std::net::Ipv6Addr,
}

derive_into_for_package!(RemCall);

#[derive(Debug, Eq, PartialEq, Clone, binserde_derive::Serialize, binserde_derive::Deserialize)]
#[cfg_attr(feature = "serde_serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serde_deserialize", derive(serde::Deserialize))]
pub struct RemAck {}

derive_into_for_package!(RemAck);

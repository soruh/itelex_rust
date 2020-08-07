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

#[derive(Copy, Clone, Eq, PartialEq, Debug, thiserror::Error)]

pub enum ServerError {
    #[error("Failed to parse package of type {}.", _0)]
    ParseFailure(u8),

    #[error("Failed to serialize package of type {}.", _0)]
    SerializeFailure(u8),
}

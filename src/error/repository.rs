#[derive(Debug, thiserror::Error)]
pub enum UrlParseError {
    #[error("invalid host")]
    InvalidHost,

    #[error("unspecified owner and repository")]
    Unspecified,

    #[error("unspecified owner")]
    UnspecifiedOwner,

    #[error("unspecified repository")]
    UnspecifiedRepository,
}

#[derive(Debug, thiserror::Error)]
pub enum Unreachable {
    #[error("unimplemented string")]
    UnimplementedString,
}

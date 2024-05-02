pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);

#[derive(Debug, thiserror::Error)]
pub enum InvalidRepositoryUrl {
    #[error("cannot be base")]
    CannotBeBase,

    #[error("cannot find owner")]
    CannotFindOwner,

    #[error("cannot find repo")]
    CannotFindRepo,
}

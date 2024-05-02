#[derive(Debug, thiserror::Error)]
pub enum InvalidRepositoryUrl {
    #[error("cannot be base")]
    CannotBeBase,

    #[error("cannot find owner")]
    CannotFindOwner,

    #[error("cannot find repo")]
    CannotFindRepo,
}

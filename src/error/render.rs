#[derive(Debug, thiserror::Error)]
pub enum Unreachable {
    #[error("DOM may be changed")]
    DomMaybeChanged,

    #[error("Undefined context")]
    UndefinedContext,
}

#[derive(Debug, thiserror::Error)]
pub enum BrowserError {
    #[error("Failed match media query")]
    FailedMatchMediaQuery,

    #[error("Null media query list")]
    NullMediaQueryList,
}

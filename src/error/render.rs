#[derive(Debug, thiserror::Error)]
pub enum Unreachable {
    #[error("DOM may be changed")]
    DomMaybeChanged,
}

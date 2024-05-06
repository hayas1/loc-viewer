#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("Option length should be lower than one")]
    OptionLengthShouldBeLowerThanOne,
}

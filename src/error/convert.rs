#[derive(Debug, thiserror::Error)]
pub enum Unreachable {
    #[error("Struct should be convert to value")]
    StructShouldBeConvertToValue,

    #[error("Query should be convert to params")]
    QueryShouldBeConvertToParams,

    #[error("Params should be convert to query")]
    ParamsShouldBeConvertToQuery,
}

#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("Option length should be lower than one")]
    OptionLengthShouldBeLowerThanOne,
}

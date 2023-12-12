use nom_locate::LocatedSpan;

use crate::parser::ParserVerboseError;

/// helper function to avoid .map_err everywhere
pub fn map_parse_error<'a, T, E, F>(
    i: LocatedSpan<&'a str>,
    result_fn: F,
    err_msg: Option<&'a str>
) -> Result<T, nom::Err<ParserVerboseError>>
where
    E: std::fmt::Display,
    F: FnOnce() -> Result<T, E>,
{
    result_fn().map_err(|e| {
        nom::Err::Failure(ParserVerboseError {
            line: i.location_line(),
            column: i.get_column(),
            input: i.fragment().to_string(),
            msg: err_msg.map(|s| s.to_string()).unwrap_or(e.to_string()),
        })
    })
}

pub fn setup_logger() {
    pretty_env_logger::init();
}

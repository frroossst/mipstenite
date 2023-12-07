use nom_locate::LocatedSpan;
use crate::parser::ParserVerboseError;
use crate::err_util::map_parse_error;

/*
  if arguments.len() != 2 {
                return map_parse_error(
                    i,
                    || Err(nom::Err::Failure(ParserVerboseError {
                        line: i.location_line(),
                        column: i.get_column(),
                        input: i.fragment().to_string(),
                        msg: format!(
                            "invalid number of arguments for instruction: {}, expected 2, got {}",
                            instruction,
                            arguments.len()
                        ),
                    })),
                    None
                )?;
            }


The following helper is meant to make the above code a single line

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

 */

pub fn check_argument_counts(args: &Vec<String>, expected: usize, i: LocatedSpan<&str>) -> Result<(), nom::Err<ParserVerboseError>> {
    // map_parse_error takes in i: LocatedSpan<&'a str>, result_fn: F, err_msg: Option<&'a str>
    // where F: FnOnce() -> Result<T, E>

    // this function should check is args.len() == expected if not then call on map_parse_error
    // and then return and propogate the error upwards to be handled by the caller

    map_parse_error(
        i,
        || {
            if args.len() != expected {
                return Err(nom::Err::Failure(ParserVerboseError {
                    line: i.location_line(),
                    column: i.get_column(),
                    input: i.fragment().to_string(),
                    msg: Default::default(),
                }));
            }
            Ok(())
        },
        Some(&format!("expected {} arguments, got {}", expected, args.len()))
    )
}
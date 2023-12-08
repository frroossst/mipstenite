use nom_locate::LocatedSpan;
use crate::parser::ParserVerboseError;
use crate::err_util::map_parse_error;
use crate::registers::{Register, register_to_addr};

/// this function should check is args.len() == expected if not then call on map_parse_error
/// and then return and propogate the error upwards to be handled by the caller
pub fn check_argument_counts(args: &Vec<String>, expected: usize, i: LocatedSpan<&str>) -> Result<(), nom::Err<ParserVerboseError>> {

    let actual = args.len();
    map_parse_error(
        i,
        || {
            if actual != expected {
                return Err(nom::Err::Failure(ParserVerboseError {
                    line: i.location_line(),
                    column: i.get_column(),
                    input: i.fragment().to_string(),
                    msg: Default::default(),
                }));
            }
            Ok(())
        },
        Some(&format!("expected {expected} arguments, got {actual}"))
    )
}

/// ensure that the argument is a valid register
/// not only check if it starts with $ but also check if it is a valid register
/// these are all the valid registers: $zero, $at, $gp, $sp, $fp, $ra, 
/// other registers are [t0-t9, s0-s7, a0-a3, v0-v1, k0-k1]
pub fn ensure_register(arg: &str, i: LocatedSpan<&str>) -> Result<(), nom::Err<ParserVerboseError>> {

    map_parse_error(
        i,
        || {
            if register_to_addr(arg.to_string()).is_none() {
                return Err(nom::Err::Failure(ParserVerboseError {
                    line: i.location_line(),
                    column: i.get_column(),
                    input: i.fragment().to_string(),
                    msg: format!("expected register, got {arg}"),
                }));
            }
            Ok(())
        },
        Some(&format!("{arg} is not a valid register"))
    )
}

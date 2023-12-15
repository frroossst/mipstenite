use nom::{
    IResult,
    error::ParseError,
    sequence::{pair, tuple},
    bytes::complete::is_not, character::complete::char, combinator::{map, recognize}, multi::separated_list1,
};
use nom_locate::LocatedSpan;

use crate::parser_utils::{check_argument_counts, ensure_register};

use super::bytecode::AsmInstruction;
use super::err_util::map_parse_error;

type Span<'a> = LocatedSpan<&'a str>;
#[derive(Debug, Clone)]
pub struct ParserVerboseError {
    pub line: u32,
    pub column: usize,
    pub input: String,
    pub msg: String,
}

impl<'a> ParseError<Span<'a>> for ParserVerboseError {
    fn from_error_kind(input: Span, kind: nom::error::ErrorKind) -> Self {
        ParserVerboseError {
            line: input.location_line(),
            column: input.get_column(),
            input: input.fragment().to_string(),
            msg: kind.description().to_string(),
        }
    }

    fn append(input: Span, kind: nom::error::ErrorKind, other: Self) -> Self {
        ParserVerboseError {
            line: input.location_line(),
            column: input.get_column(),
            input: input.fragment().to_string(),
            msg: format!("{}: {}", kind.description(), other.msg),
        }
    }
}

impl From<nom::Err<ParserVerboseError>> for ParserVerboseError {
    fn from(err: nom::Err<ParserVerboseError>) -> Self {
        match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => e,
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
}

/// mips supports # comments only
fn eol_comment<'a>(i: Span<'a>) -> IResult<Span<'a>, String, ParserVerboseError> {
    let (i, (_, comment)) = pair(char('#'), is_not("\r\n"))(i).map_err(|_: nom::Err<nom::error::Error<Span<'a>>>| {
        nom::Err::Failure(ParserVerboseError {
            line: i.location_line(),
            column: i.get_column(),
            input: i.fragment().to_string(),
            msg: "failed to parse comment".to_string(),
        })
    })?;
    Ok((i, comment.fragment().to_string()))
}

/// consumes all whitespace characters
fn consume_whitespace<'a>(i: Span<'a>) -> IResult<Span<'a>, (), ParserVerboseError> {
    let (i, _) = nom::character::complete::multispace0(i)?;
    Ok((i, ()))
}

/// function to parse a line of assembly instructions
fn parse_instruction<'a>(i: Span<'a>) -> IResult<Span<'a>, AsmInstruction, ParserVerboseError> {
    let stripped_src = consume_whitespace(i)?.0;
    // parse instruction, parse alphabet until space and return the rest of the source
    let (args, ins) = recognize(tuple((is_not(" "), consume_whitespace)))(stripped_src)?;
    let instruction = ins.fragment().trim().to_string();

    // parse arguments until newline, each argument is separated by a comma
    // any amount of whitespace is allowed between arguments
    let (i, arguments) = separated_list1(
        char(','),
        map(
            recognize(tuple((consume_whitespace, is_not(",\r\n"), consume_whitespace))),
            |s: Span| s.fragment().trim().to_string(),
        ),
    )(args)?;

    let remaining = consume_whitespace(i)?.0;

    match instruction.as_str() {
        "li" => {
            check_argument_counts(&arguments, 2, i)?;
            let reg = arguments.get(0).unwrap();
            ensure_register(reg, i)?;
            let imm = map_parse_error(i, || arguments.get(1).unwrap().parse::<i16>(), Some("unable to parse immediate value"))?;
            Ok((remaining, AsmInstruction::LI(reg.to_string(), imm)))
        }
        "add" => {
            check_argument_counts(&arguments, 3, i)?;
            let rd = arguments.get(0).unwrap();
            ensure_register(rd, i)?;
            let rs = arguments.get(1).unwrap();
            ensure_register(rs, i)?;
            let rt = arguments.get(2).unwrap();
            ensure_register(rt, i)?;
            Ok((remaining, AsmInstruction::ADD(rd.to_string(), rs.to_string(), rt.to_string())))
        }
        // else return error
        _ => Err(nom::Err::Failure(ParserVerboseError {
            line: i.location_line(),
            column: i.get_column(),
            input: i.fragment().to_string(),
            msg: format!("invalid instruction: {instruction}"),
        })),
    }

}

pub fn mock_parser(src_in: &str) -> Result<(LocatedSpan<&str>, Vec<AsmInstruction>), nom::Err<ParserVerboseError>> {

    let mut bytecode_source = Vec::new();

    let (text_section, data_section) = match (src_in.find(".text"), src_in.find(".data")) {
        (Some(text_index), Some(data_index)) if data_index > text_index => {
            let text_section = &src_in[text_index..data_index].trim()[..];
            let data_section = &src_in[data_index..].trim()[..];
            (text_section, data_section)
        }
        (Some(text_index), None) => {
            let text_section = &src_in[text_index..].trim()[..];
            ("", text_section)
        }
        (None, Some(data_index)) => {
            let data_section = &src_in[data_index..].trim()[..];
            (data_section, "")
        }
        _ => ("", "")
    };

    if text_section == "" {
        return Err(nom::Err::Failure(ParserVerboseError {
            line: 0,
            column: 0,
            input: src_in.to_string(),
            msg: "missing .text section".to_string(),
        }));
    }

    let mut text_input_source = Span::new(text_section);

    loop {
        match parse_instruction(text_input_source) {
            Ok((remaining, parsed_result)) => {
                bytecode_source.push(parsed_result);

                if remaining.fragment().is_empty() {
                    return Ok((remaining, bytecode_source));
                }

                // Update the remaining input for the next iteration
                text_input_source = remaining;
            }
            Err(err) => {
                eprintln!("Parsing error: {:?}", err);
                return Err(err);
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eol_comment() {
        let input = "# This is a comment\n";
        let result = eol_comment(Span::new(input));
        assert!(result.is_ok());
        let (i, comment) = result.unwrap();
        assert_eq!(i.fragment(), &"\n");
        assert_eq!(comment, " This is a comment");

        let input = "// This is NOT a comment";
        let result = eol_comment(Span::new(input));
        assert!(result.is_err());

        let err = result.unwrap_err();
        let pve: ParserVerboseError = err.into();
        assert!(pve.msg.contains("failed to parse comment"));
    }

    #[test]
    fn test_parse_instruction() {
        let input = "li $t1, 45";
        let result = parse_instruction(Span::new(input));
        assert!(result.is_ok());
        let (_, instruction) = result.unwrap();
        assert_eq!(instruction, AsmInstruction::LI("$t1".to_string(), 45));

        let input = "li $t1, -6";
        let result = parse_instruction(Span::new(input));
        assert!(result.is_ok());
        let (_, instruction) = result.unwrap();
        assert_eq!(instruction, AsmInstruction::LI("$t1".to_string(), -6));
    }


    #[test]
    fn test_parse_instruction_multiline() {
        let input = "li $t1, 45\nadd $t1, $t1, $t1";
        let result = parse_instruction(Span::new(input));
        assert!(result.is_ok());
        let (i, instruction) = result.unwrap();
        assert_eq!(i.fragment(), &"add $t1, $t1, $t1");
        assert_eq!(instruction, AsmInstruction::LI("$t1".to_string(), 45));

        let result = parse_instruction(i);
        assert!(result.is_ok());
        let (i, instruction) = result.unwrap();
        assert_eq!(i.fragment(), &"");
        assert_eq!(instruction, AsmInstruction::ADD("$t1".to_string(), "$t1".to_string(), "$t1".to_string()));

    }

    #[test]
    fn test_parse_text_section() {
        let input = r#".text
        li $t0, 1
        li $t1, 9"#;

        let result = parse_instruction(Span::new(input));
    }

    #[test]
    fn test_integrated_all() {
        let _src = r#"
        # Program File: Program2-1.asm 
        # Author: Charles Kann
        # Purpose: First program, Hello World
        .text                   # Define the program instructions.
        main:                   # Label to define the main program.
            li $v0,4            # Load 4 into $v0 to indicate a print string.
            la $a0, greeting    # Load the address of the greeting into $a0.
            syscall             # Print greeting. The print is indicated by
                                # $v0 having a value of 4, and the string to
                                # print is stored at the address in $a0.
            li $v0, 10          # Load a 10 (halt) into $v0.
            syscall             # The program ends.
        .data                   # Define the program data.
        greeting: .asciiz "Hello World" #The string to print.
        "#;
        unimplemented!("integrated tests")
    }


}

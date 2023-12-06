use nom::{
    IResult,
    error::ParseError,
    sequence::pair,
    bytes::complete::is_not, character::complete::char,
};
use nom_locate::LocatedSpan;

use super::bytecode::AsmInstruction;
use std::collections::BTreeSet;

type Span<'a> = LocatedSpan<&'a str>;
#[derive(Debug, Clone)]
struct ParserVerboseError {
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

/// function to parse a line of assembly instructions
fn parse_instruction<'a>(i: Span<'a>) -> IResult<Span<'a>, AsmInstruction, ParserVerboseError> {
    Ok((i, AsmInstruction::LI("$t1".to_string(), 45)))
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
        let (i, instruction) = result.unwrap();
        assert_eq!(i.fragment(), &"");
        assert_eq!(instruction, AsmInstruction::LI("$t1".to_string(), 45));
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

use cssparser::{ParseError, Parser, ParserInput, ToCss, Token};
use std::fmt::Write;

const PREFIX_SEPARATOR: &str = "-";

pub struct CustomError {
    message: String,
}

impl std::fmt::Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

struct ParseState<'a> {
    output: &'a mut String,
}

impl<'a> ParseState<'a> {
    fn new(output: &'a mut String) -> Self {
        Self { output }
    }

    fn sub<'b>(&'b mut self) -> ParseState<'b>
    where
        'a: 'b,
    {
        ParseState {
            output: self.output,
        }
    }
}

pub fn compile<'a>(style: &'a str, prefix: &'a str) -> Result<String, ParseError<'a, CustomError>> {
    let mut parser_input = ParserInput::new(&style);
    let mut parser = Parser::new(&mut parser_input);
    let mut output = String::new();
    let state = ParseState::new(&mut output);
    match parse_segment(&mut parser, state, prefix) {
        Ok(_) => Ok(output),
        Err(err) => Err(err),
    }
}

fn parse_segment<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    mut state: ParseState<'a>,
    prefix: &'a str,
) -> Result<(), ParseError<'i, CustomError>> {
    while !parser.is_exhausted() {
        parse_block(parser, state.sub(), prefix).unwrap();
    }
    Ok(())
}

fn parse_block<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    mut state: ParseState<'a>,
    prefix: &'a str,
) -> Result<(), ParseError<'i, CustomError>> {
    parse_common_block(parser, state.sub(), prefix)
}

fn parse_common_block<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    mut state: ParseState<'a>,
    prefix: &'a str,
) -> Result<(), ParseError<'i, CustomError>> {
    while !parser.is_exhausted() {
        let next = parser.next().unwrap().clone();
        // let next = parser.next_including_whitespace().unwrap().clone();
        println!("Next {:?}", next);
        match next {
            Token::Ident(ref _s) => write_token(&next, state.output),
            Token::Delim(c) => {
                // TODO: deal with namespace by uuid here.
                write_token(&next, state.output);
                println!("Delimmmmm {:?}", c);
                if &c.to_string() == "." {
                    state
                        .output
                        .write_str(&(prefix.to_owned() + PREFIX_SEPARATOR))
                        .unwrap();
                }
            }
            Token::Colon => {
                write_token(&next, state.output);
            }
            Token::Semicolon => {
                write_token(&next, state.output);
            }
            Token::Comma => {
                write_token(&next, state.output);
            }
            Token::Function(_) => {
                write_token(&next, state.output);
                parser
                    .parse_nested_block(|parser| parse_segment(parser, state.sub(), prefix))
                    .unwrap();
                state.output.write_str(")").unwrap();
            }
            Token::ParenthesisBlock => {
                state.output.write_str("(").unwrap();
                parser
                    .parse_nested_block(|parser| parse_segment(parser, state.sub(), prefix))
                    .unwrap();
                state.output.write_str(")").unwrap();
            }
            Token::SquareBracketBlock => {
                state.output.write_str("[").unwrap();
                parser
                    .parse_nested_block(|parser| parse_segment(parser, state.sub(), prefix))
                    .unwrap();
                state.output.write_str("]").unwrap();
            }
            Token::CurlyBracketBlock => {
                state.output.write_str("{").unwrap();
                parser
                    .parse_nested_block(|parser| parse_segment(parser, state.sub(), prefix))
                    .unwrap();
                state.output.write_str("}").unwrap();
                // state.output.write_str("\n}").unwrap();
            }
            token => {
                write_token(&token, state.output);
            } // _ => return Err(parser.new_unexpected_token_error(next)),
        }
    }
    Ok(())
}

fn write_token(token: &Token, output: &mut String) -> () {
    token.to_css(output).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn it_works() {
        let style = r#"
.hello {
    color: white;
    background-color: black;
    width: 100%;
}"#;
        let output = parser::compile(style, "Hello");
        println!("==========================");
        println!("Test Output!!! {:?}", output);
    }
}

use anyhow;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseError {
    details: String
}
impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError{details: msg.to_string()}
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn get_int(input: &str) -> anyhow::Result<i32> {
    Ok(parse_into_int(input)?)
}

fn parse_into_int(input: &str) -> Result<i32, ParseError> {
    match input.parse::<i32>() {
        Ok(result) => Ok(result),
        Err(e) => Err(ParseError::new(format!("could not parse {} to int: {}", input, e).as_str()))
    }
}

fn main() {
    println!("{}", get_int("32").unwrap());
    println!("{}", get_int("this is not an integer").unwrap());
}
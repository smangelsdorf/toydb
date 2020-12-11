use nom::{branch::*, bytes::streaming::*, combinator::*, error::VerboseError};

use crate::query::ast;

pub fn parse_sql(input: &str) -> Result<(&str, ast::Query), nom::Err<VerboseError<&str>>> {
    alt((
        map(tag_no_case("select"), |_: &str| ast::Query::Select),
        map(tag_no_case("insert"), |_: &str| ast::Query::Insert),
    ))(input)
}

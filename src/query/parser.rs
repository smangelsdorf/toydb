use std::str::FromStr;

use nom::{
    branch::*, bytes::streaming::*, character::streaming::*, character::*, combinator::*,
    error::VerboseError, multi::*, sequence::*, IResult,
};

use crate::query::ast;

pub fn parse_sql(input: &str) -> Result<(&str, ast::Query), nom::Err<VerboseError<&str>>> {
    alt((
        map(tag_no_case("select"), |_: &str| ast::Query::Select),
        map(
            tuple((
                tag_no_case("insert"),
                space1,
                map_res(digit1, u64::from_str),
                space1,
                recognize(many1(satisfy(|c| !c.is_whitespace()))),
                space1,
                recognize(many1(satisfy(|c| !c.is_whitespace()))),
            )),
            |(_, _, id, _, username, _, email)| ast::Query::Insert(id, username, email),
        ),
    ))(input)
}

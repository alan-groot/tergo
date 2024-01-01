use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};
use nom::{
    bytes::complete::tag,
    sequence::{preceded, tuple},
    IResult,
};

use crate::ast::CompoundStatement;
use crate::expression::{condition, identifier};
use crate::helpers::CodeSpan;
use crate::{ast::Expression, expression::expr};

pub fn repeat(input: CodeSpan) -> IResult<CodeSpan, Box<Expression>> {
    map(
        preceded(tuple((tag("repeat"), multispace0)), expr),
        |repeat_expr| Box::new(Expression::Compound(CompoundStatement::Repeat(repeat_expr))),
    )(input)
}

pub fn while_stmt(input: CodeSpan) -> IResult<CodeSpan, Box<Expression>> {
    map(
        tuple((preceded(tag("while"), condition), expr)),
        |(cond, while_expr)| {
            Box::new(Expression::Compound(CompoundStatement::While(
                cond, while_expr,
            )))
        },
    )(input)
}

pub fn for_stmt(input: CodeSpan) -> IResult<CodeSpan, Box<Expression>> {
    fn for_cond(input: CodeSpan) -> IResult<CodeSpan, (Box<Expression>, Box<Expression>)> {
        delimited(
            tuple((
                multispace0,
                nom::character::complete::char('('),
                multispace0,
            )),
            separated_pair(
                identifier,
                tuple((multispace0, tag("in"), multispace0)),
                expr,
            ),
            tuple((
                multispace0,
                nom::character::complete::char(')'),
                multispace0,
            )),
        )(input)
    }

    map(
        tuple((tag("for"), for_cond, expr)),
        |(_, (symbol, cond_expr), for_expr)| {
            Box::new(Expression::Compound(CompoundStatement::For(
                symbol, cond_expr, for_expr,
            )))
        },
    )(input)
}

mod tests {
    use crate::ast::Literal;

    use super::*;

    #[test]
    fn test_repeat() {
        let tests = [
            (
                "repeat TRUE",
                Box::new(Expression::Compound(CompoundStatement::Repeat(Box::new(
                    Expression::Literal(Literal::True),
                )))),
            ),
            (
                r#"repeat
        {}"#,
                Box::new(Expression::Compound(CompoundStatement::Repeat(Box::new(
                    Expression::Expressions(vec![]),
                )))),
            ),
        ];
        for (input, expected) in tests {
            let input = CodeSpan::new(input);
            assert_eq!(repeat(input).unwrap().1, expected);
        }
    }

    #[test]
    fn test_while() {
        let tests = [(
            "while(TRUE)FALSE",
            Box::new(Expression::Compound(CompoundStatement::While(
                Box::new(Expression::Literal(Literal::True)),
                Box::new(Expression::Literal(Literal::False)),
            ))),
        )];
        for (input, expected) in tests.clone() {
            let input = CodeSpan::new(input);
            assert_eq!(while_stmt(input).unwrap().1, expected);
        }

        let input_with_nl = r#"while
        (TRUE)
        FALSE"#;
        assert_eq!(
            while_stmt(CodeSpan::new(tests[0].0)).unwrap().1,
            while_stmt(CodeSpan::new(input_with_nl)).unwrap().1
        );
    }

    #[test]
    fn test_for() {
        let input = "for(a in TRUE) TRUE";
        let expected = Box::new(Expression::Compound(CompoundStatement::For(
            Box::new(Expression::Identifier("a".to_string())),
            Box::new(Expression::Literal(Literal::True)),
            Box::new(Expression::Literal(Literal::True)),
        )));
        assert_eq!(for_stmt(CodeSpan::new(input)).unwrap().1, expected);

        let input_with_nl = r#"for 
        (
        a 
        in 
        TRUE
        )
        TRUE"#;
        assert_eq!(
            for_stmt(CodeSpan::new(input)).unwrap().1,
            for_stmt(CodeSpan::new(input_with_nl)).unwrap().1
        );
    }
}

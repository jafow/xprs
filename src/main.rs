#[macro_use]
extern crate slog;
extern crate sloggers;

use slog::Logger;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;


#[derive(Debug, Clone, Eq, PartialEq)]
enum Operation {
    Root,
    Plus,
    Minus,
    Null
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Expression {
    opr: Operation,
    lhs: String,
    rhs: Vec<Expression>
}

impl Expression {
    fn new(input: &str, logger: &Logger) -> Expression {

        let op: Operation = set_operator(input, logger);

        Expression {
            opr: op,
            lhs: input['a'.len_utf8()..].to_string(),
            rhs: vec![]
        }
    }
}

fn set_operator(input: &str, logger: &Logger) -> Operation {
    let opr: Operation = match input.chars().next() {
        Some(next) => {
            match next {
                '√' => Operation::Root,
                '+' => Operation::Plus,
                '-' => Operation::Minus,
                _ => {
                    error!(logger, "Operation error: cannot parse given operator.");
                    Operation::Null
                }
            }
        },
        _ => {
            error!(logger, "Operator err: no operator given.");
            Operation::Null
        }
    };
    opr
} 



fn expr_builder(inp: &str, logger: &Logger) -> Result<Expression, String> {
    match inp.chars().next() {
        Some('(') => {
            let rest = &inp['('.len_utf8()..];
            let exp = Expression::new(rest, logger);
            Ok(exp)
        },
        Some(')') => {
            debug!(logger, "end of the line");
            Err("end of the line".to_string())
        },
        _ => {
            debug!(logger, "missing stuff");
            Err("missing stuff".to_string())
        }
    }
}

fn main() {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);

    let logger = builder.build().unwrap();

    let xp = expr_builder("(+ 8 3)", &logger);
    let expected: Expression = Expression { opr: Operation::Plus, lhs: String::from(" 8 3)"), rhs: vec![] };

    debug!(logger, "expression {:?}", xp);
    assert_eq!(Ok(expected), xp)
}

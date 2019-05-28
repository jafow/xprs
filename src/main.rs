#[macro_use]
extern crate slog;
extern crate sloggers;

use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;


#[derive(Debug)]
struct Rational {
    numerator: f32,
    denominator: f32
}

enum Operation {
    Root,
    Plus,
    Minus
}

struct Expression {
    opr: Operation,
    lhs: i32,
    rhs: Box<Expression>
}

// fn prs(piece: &str) -> Result<(&str, String), &str> {
//     let mut inpchar = piece.char_indices();
//     let mut oper = String::new();
//     let mut tokens: Vec<&str> = Vec::new();

//     // check for empty or start of expression
//     match inpchar.next() {
//         Some((idx, next)) if next == '(' => prs(&piece[idx..]),
//         _ => Err(piece)
//     }

    
//     // while let Some(next) = inpchar.next() {
//     //     match next {
//     //         '√' => Operation::Root,
//     //         '+' => Operation::Plus,
//     //         '-' => Operation::Minus,
//     //         _ => return Err(piece)
//     //     };
//     // }

//     Ok((&piece[..], oper))
// }

// fn prs(piece: &str) -> Result<(Vec<String>, &str), &str> {
//     let mut builder = TerminalLoggerBuilder::new();
//     builder.level(Severity::Debug);
//     builder.destination(Destination::Stderr);

//     let logger = builder.build().unwrap();

//     let mut expr_chunk = Vec::new();
//     let mut inpchar = piece.chars();

//     let mut current_token = String::new();

//     match inpchar.next() {
//         Some(next) => Operation::root
        
//         Some(next) if next == '(' => return Ok(inpc),
//         Some(next) if next == ' ' => current_token.push(inpchar.next()),
//         _ => return Err(piece),
//     }

//     while let Some(next) = inpchar.next() {
//         // println!("next {:?}", next);
//         if next == ')' || next == ' ' {
//             break;
//         } else {
//             expr_chunk.push(next)
//         }
//     }
//     let next_idx = expr_chunk.len();
//     debug!(logger, "next idx: {:?}", next_idx);
    
//     Ok((expr_chunk, piece))
// }

fn main() {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);

    let logger = builder.build().unwrap();

    // let inp: &str = "(+ 8 3)";

    // let x = prs(inp);
    info!(logger, "yo: {:?}", x);
    // println!("x {:?}", x);
}

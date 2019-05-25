#[derive(Debug)]
struct Rational {
    numerator: f32,
    denominator: f32
}

fn prs(piece: &str) -> Result<(Vec<String>, &str), &str> {
    let mut express_chunk = Vec::new();
    let mut inpchar = piece.chars();

    let mut current_token = String::new();
    match inpchar.next() {
        Some(next) if next == '(' => current_token.push(next),
        _ => return Err(piece),
    }

    while let Some(next) = inpchar.next() {
        if next == ')' {
            break;
        } else if next == ' ' {
            // new word
            express_chunk.push(current_token);
            current_token = String::new();
        } else {
            current_token.push(next)
        }
    }
    // let next_idx = express_chunk.len();
    Ok((express_chunk, piece))
}

fn main() {
    /// example input
    /// (rt 8 3) 2.0
    let inp: &str = "(rt 8 3)";

    let x = prs(inp);

    println!("x {:?}", x);
}

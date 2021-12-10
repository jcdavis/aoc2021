use std::env;
use std::fs;

fn first_illegal(string: &str) -> (Option<char>, Vec<char>) {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return (Some(c), stack);
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return (Some(c), stack);
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return (Some(c), stack);
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return (Some(c), stack);
                }
            }
            _ => panic!("unexpected char {}", c),
        }
    }
    (None, stack)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut scores = Vec::new();
    for line in input.lines() {
        let (missing, stack) = first_illegal(line);

        if missing == None {
            let mut line_score: i64 = 0;
            for c in stack.iter().rev() {
                line_score *= 5;
                line_score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("{}", c),
                }
            }
            scores.push(line_score);
        }
    }
    scores.sort();
    println!("{:?}", scores[scores.len()/2]);
}

use std::env;
use std::fs;

fn first_illegal(string: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return Some(c);
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Some(c);
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return Some(c);
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return Some(c);
                }
            }
            _ => return Some('?'),
        }
    }
    None
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut score = 0;
    for line in input.lines() {
        score += match first_illegal(line) {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            None => 0,
            _ => -10000000,
        }
    }
    println!("{:?}", score);
}

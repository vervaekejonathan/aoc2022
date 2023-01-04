use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    let (start_position, commands) = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut lines: Vec<_> = start_position.split("\n").map(|line| { line }).collect();
    lines.reverse();
    let mut stack : Vec<VecDeque<String>> = Vec::new();
    for _ in 0 .. 9 {        
        stack.push(VecDeque::<String>::new());
    }

    for line in &lines {
        for i in 0 .. 9 {
            decode_stack(&mut stack, i, line);
        }
    }

    for line in commands.split("\n") {
        let (total, src, dst)
            = line.split(" ")
                  .skip(1)
                  .step_by(2)
                  .map(|x| 
                        x.parse::<i32>().unwrap())
                  .collect_tuple()
                  .unwrap();

        for _ in 0..total {
            let v = stack[(src - 1) as usize].pop_back().unwrap();
            stack[(dst - 1) as usize].push_back(v);
        }
    }  

    stack.iter().for_each(|s| print!("{}", s.back().unwrap() ) );
    println!();

}

fn decode_stack(stack : &mut Vec<VecDeque<String>>, i : usize, line : &str) {
    let letter = line.chars().nth((i * 4) + 1).unwrap();
    if letter.is_alphabetic() {
        stack[i].push_back(letter.to_string());
    }
}
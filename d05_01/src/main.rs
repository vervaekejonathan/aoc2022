use std::collections::VecDeque;

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
        let line = line.replace("move ", "");
        let (total, (src, dst)) = line.split_once(" from ")
                                 .map(|(l,r)| 
                                        ( l.parse::<usize>().unwrap(), 
                                          r.split_once(" to ")
                                           .map(|(l,r)| 
                                                ( l.parse::<usize>().unwrap() - 1,
                                                  r.parse::<usize>().unwrap() - 1)
                                                ).unwrap()
                                        )
                                    )
                                 .unwrap();

        for _ in 0..total {
            let v = stack[src].pop_back().unwrap();
            stack[dst].push_back(v);
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
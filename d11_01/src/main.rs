use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Debug)]
struct Monkey<'a> {
    items: VecDeque<i64>,
    operation: &'a str,
    divisible: i64,
    on_true: usize,
    on_false: usize,
    actions: u32,
}

fn main() {
    let mut monkeys = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey| {            
            let lines = monkey.split("\n").collect::<Vec<_>>();
            let (_, items) = lines[1].split_once(":").unwrap();
            let (_, operation) = lines[2].split_once("=").unwrap();
            let (_, divisible) = lines[3].split_once("by").unwrap();
            let (_, on_true) = lines[4].split_once("monkey").unwrap();
            let (_, on_false) = lines[5].split_once("monkey").unwrap(); 
            Monkey { 
                items: items.split(",")
                            .map(
                                |x| {
                                x.trim().parse::<i64>().unwrap() 
                            })
                            .collect::<VecDeque<i64>>(),
                operation: operation,
                divisible: divisible.trim().parse::<i64>().unwrap(),
                on_true: on_true.trim().parse::<usize>().unwrap(),
                on_false: on_false.trim().parse::<usize>().unwrap(),
                actions: 0,
            }
        }
    ).collect::<Vec<Monkey>>();

    for i in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            while monkeys[monkey_idx].items.len() > 0 {
                let item = monkeys[monkey_idx].items.pop_front().unwrap();
                let expr : meval::Expr = monkeys[monkey_idx].operation.parse().unwrap();
                let level = expr.bind("old").unwrap()(item as f64);
                let bored_level = (level / 3.0) as i64;
                if bored_level % monkeys[monkey_idx].divisible == 0 {
                    let monkey = monkeys[monkey_idx].on_true;
                    monkeys[monkey].items.push_back(bored_level);
                } else {
                    let monkey = monkeys[monkey_idx].on_false;
                    monkeys[monkey].items.push_back(bored_level);
                }
                monkeys[monkey_idx].actions += 1;
            }
        }
    }

    println!("{}", monkeys.iter().map(|x| x.actions)
                          .sorted_unstable_by(|a, b| b.cmp(a))
                          .take(2)
                          .product::<u32>());

}

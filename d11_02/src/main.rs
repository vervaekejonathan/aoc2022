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

    let modder : i64 = monkeys.iter().map(|monkey| monkey.divisible).product();

    for _ in 0..100 {
        for monkey_idx in 0..monkeys.len() {
            while monkeys[monkey_idx].items.len() > 0 {
                let monkey = &mut monkeys[monkey_idx];
                let mut item = monkey.items.pop_front().unwrap();
                item = (monkey.operation.parse::<meval::Expr>().unwrap().bind("old").unwrap()(item as f64) as i64) % modder;
                if item % monkey.divisible == 0 {
                    let monkey_send = monkey.on_true;
                    monkeys[monkey_send].items.push_back(item);
                } else {
                    let monkey_send = monkey.on_false;
                    monkeys[monkey_send].items.push_back(item);
                }   
                monkeys[monkey_idx].actions += 1;
            }            
        }
    }

    let (p,q) = monkeys.iter().map(|x| x.actions)
                          .sorted_unstable_by(|a, b| b.cmp(a))
                          .take(2).collect_tuple().unwrap();
    println!("{}", p as u64 * q as u64);

}

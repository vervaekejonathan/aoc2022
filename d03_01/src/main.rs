fn main() {
    let contents = include_str!("input.txt").split("\n");
    let score : Vec<i32> = contents.map(|x| decode(x)).collect();
    println!("{:?}", score.into_iter().sum::<i32>());
}

fn decode(text : &str) -> i32 {
    let (part1, part2) = text.split_at(text.len() / 2);
    let mut non_unique = part1.chars().filter(|x| part2.contains(*x)).collect::<Vec<_>>();
    non_unique.dedup();  
    let transformed : Vec<_> = non_unique.into_iter()
                               .map(|x| if x >= 'a' 
                                                { (x as i32) - ('a' as i32) + 1} 
                                              else 
                                                { (x as i32) - ('A' as i32) + 27})
                               .collect();
    transformed.into_iter().sum()
}
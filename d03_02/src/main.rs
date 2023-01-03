fn main() {
    let contents = include_str!("input.txt").split("\n").collect::<Vec<_>>();
    let score : Vec<i32> = contents.chunks(3).map(|set| decode(set[0], set[1], set[2])).collect();
    println!("{:?}", score.into_iter().sum::<i32>());
}

fn decode(part1 : &str, part2 : &str, part3 : &str) -> i32 {
    let mut non_unique = part1.chars().filter(|x| part2.contains(*x) && part3.contains(*x)).collect::<Vec<_>>();
    non_unique.dedup();  
    let transformed : Vec<_> = non_unique.into_iter()
                               .map(|x| if x >= 'a' 
                                                { (x as i32) - ('a' as i32) + 1} 
                                              else 
                                                { (x as i32) - ('A' as i32) + 27})
                               .collect();
    transformed.into_iter().sum()
}
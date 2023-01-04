fn main() {
    let contents = include_str!("input.txt").split("\n");
    let score : Vec<i32> = contents.map(|x| decode(x)).collect();
    println!("{:?}", score.into_iter().sum::<i32>());
}

fn decode(text : &str) -> i32 {
    //println!("{}", text);
    let p = text.split(",")
                                        .filter_map(|x| 
                                                x.split_once("-")
                                                  .and_then(|(l, r)| 
                                                        Some((l.parse::<i32>().ok().unwrap(), 
                                                              r.parse::<i32>().ok().unwrap())) )
                                        )
                                        .collect::<Vec<_>>();
    //println!("{:?}", p);
    let (part1_min,part1_max) = (p[0].0, p[0].1);
    let (part2_min,part2_max) = (p[1].0, p[1].1);
    println!("{} {} {} {}", part1_min, part1_max, part2_min, part2_max);
    let r = ((part1_min..part1_max+1).contains(&part2_min) &&
                   (part1_min..part1_max+1).contains(&part2_max)) 
                  || 
                  ((part2_min..part2_max+1).contains(&part1_min) &&
                   (part2_min..part2_max+1).contains(&part1_max));
    println!("{}", r);
    r as i32
}
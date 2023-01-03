fn main() {
    let content = include_str!("input.txt").split("\n\n");
    let mut score : Vec<i32> = content.map(|x| decode(x)).collect();
    score.sort();
    score.reverse();
    println!("{:?}", score[0]);
}

fn decode(text : &str) -> i32 {
    let summed = text.split("\n").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    summed.into_iter().sum()
}


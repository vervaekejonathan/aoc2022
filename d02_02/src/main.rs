fn main() {
    let contents = include_str!("input.txt").split("\n");
    let score : Vec<i32> = contents.map(|x| decode(x)).collect();
    println!("{:?}", score.into_iter().sum::<i32>());
}

fn decode(text : &str) -> i32 {
    let elf_play = text.chars().nth(0).unwrap() as i32;
    let you_play = text.chars().nth(2).unwrap();

    let elf_normalized = (elf_play as i32) - ('A' as i32);

    let you_normalized = match you_play {
        'X' => { (elf_normalized - 1).rem_euclid(3) },
        'Y' => { elf_normalized },
        'Z' => { (elf_normalized + 1).rem_euclid(3) },
        _ => { 0 }
    };

    let score_calculator =  || -> i32 {
        if (elf_normalized + 1) % 3 == you_normalized {
            6
        }
        else if elf_normalized == you_normalized {
            3
        }
        else {
            0
        }
    };

    let score_winner = score_calculator();

    (you_normalized + 1) + score_winner
}


fn main() {
    let (mut value, mut clock_cycle) = (1, 1);
    let mut image : Vec<char> = Vec::with_capacity(40*6);
    let mut sprite : Vec<i32> = Vec::with_capacity(40*6);

    for _ in 0..40*6 {
        image.push('.');
        sprite.push(0);
    }

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| 
        {   
            sprite[clock_cycle-1] = value;
            clock_cycle += 1; 

            if line.contains("addx") {  
                sprite[clock_cycle-1] = value;
                clock_cycle += 1;        
                let (_, operand) = line.split_once(" ")
                                        .map(
                                            |(l,r)| 
                                            (l, r.parse::<i32>().unwrap())
                                        )
                                        .unwrap();
                value += operand;                            
            }
        }  
    );

    sprite.chunks(40).for_each(|line|
        {
            line.iter().for_each(
                    |x| 
                    print!("{:0>2} ", x));
            println!(); 
        });

    for i in 0..6 {
        for j in 0..40 {
            let index = j+(40*i);
            if sprite[index] >= (j as i32) - 1 &&
               sprite[index] <= (j as i32) + 1 {
                image[index] = '#';
            }
        }
    }

    image.chunks(40).for_each(|line|
        {
            line.iter().for_each(
                    |x| 
                    print!("{}", x));
            println!(); 
        });
}

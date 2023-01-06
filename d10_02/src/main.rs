
fn main() {
    let mut value : i32 = 1;
    let mut clock_cycle : i32 = 1;
    let mut image : Vec<char> = Vec::with_capacity(40*6);

    include_str!("input.txt")
        .split("\n")
        .for_each(|line| 
        {   
            if value >= (((clock_cycle - 1 ) % 40) - 1) as i32 &&
               value <= (((clock_cycle - 1 ) % 40) + 1) as i32
            {
                image.push('#');
            } else {
                image.push('.');
            }
            clock_cycle += 1; 

            if line.contains("addx") {  
                if value >= (((clock_cycle - 1 ) % 40) - 1) as i32 &&
                   value <= (((clock_cycle - 1 ) % 40) + 1) as i32
                {
                    image.push('#');
                } else {
                    image.push('.');
                }
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

    image.chunks(40).for_each(|line|
        {
            line.iter().for_each(
                    |x| 
                    print!("{}", x));
            println!(); 
        });
}

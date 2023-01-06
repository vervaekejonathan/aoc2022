
fn main() {
    let (mut result, mut value, mut clock_cycle) = (0, 1, 1);
    include_str!("input.txt")
        .split("\n")
        .for_each(|line| 
        {                
            clock_cycle += 1; 

            let modulus = ((clock_cycle as i32) + 20) % 40;        
            if modulus <= 1 {
                result += value * (clock_cycle / 20 * 20);
            }

            if line.contains("addx") {            
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
    ) ;
    println!("{}", result);

}

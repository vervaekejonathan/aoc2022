fn main() {
    let content = include_str!("input.txt").split("\n").collect::<Vec<_>>();
    //println!("{:?}", content);

    let mut data = [[0i8; 99]; 99];
    let mut visible = [[0i8; 99]; 99];
    for i in 0..99 {
        for j in 0..99 {   
            data[i][j] = content[i]
                            .chars()
                            .nth(j)
                            .map(|x| x.to_string().parse::<i8>().unwrap())
                            .unwrap();
        }
    }

    for i in 0..99 {
        let mut heighest = -1;
        for j in 0..99 {
            let current = data[i][j];
            if current > heighest {
                visible[i][j] = 1;
                heighest = current;
            }
        }
    }

    for i in 0..99 {
        let mut heighest = -1;
        for j in (0..99).rev() {
            let current = data[i][j];
            if current > heighest {
                visible[i][j] = 1;
                heighest = current;
            }
        }
    } 
    
    for j in 0..99 {
        let mut heighest = -1;
        for i in 0..99 {
            let current = data[i][j];
            if current > heighest {
                visible[i][j] = 1;
                heighest = current;
            }
        }
    }       

    for j in 0..99 {
        let mut heighest = -1;
        for i in (0..99).rev() {
            let current = data[i][j];
            if current > heighest {
                visible[i][j] = 1;
                heighest = current;
            }
        }
    } 
    visible.into_iter().for_each(|x| println!("{:?}", x));


    let mut total = 0;
    for i in 0..99 {
        for j in 0..99 {
            total = total + (visible[i][j] == 1) as i16;
        }
    }
    println!("{}", total);
}

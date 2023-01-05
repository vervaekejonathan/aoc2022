
const MAX : usize = 99; 

fn main() {    
    let content = include_str!("input.txt").split("\n").collect::<Vec<_>>();

    let mut data = [[0i8; MAX]; MAX];
    let mut score = [[0u32; MAX]; MAX];
    for i in 0..MAX {
        for j in 0..MAX {   
            data[i][j] = content[i]
                            .chars()
                            .nth(j)
                            .map(|x| x.to_string().parse::<i8>().unwrap())
                            .unwrap();
        }
    }

    for i in 0..MAX {
        for j in 0..MAX {
            score[i][j] = lookup(&data, i, j) * lookdown(&data, i, j) * lookleft (&data, i, j) * lookright(&data, i, j);
        }
    }
    //score.into_iter().for_each(|x| println!("{:?}", x));

    let max = score
    .iter()
    .flatten()
    .copied()
    .fold(0, |prev, curr| prev.max(curr));
    println!("{}", max);

}

fn lookup(data : &[[i8; MAX]; MAX], i : usize, j : usize) -> u32 {
  let mut total = 0;
  let mine = data[i][j];
  for d in (0..i).rev() {
    let current = data[d][j];
    if current < mine {
        total = total + 1;
    } else {
            total = total  + 1;
        break;
      }

  }
  total

}

fn lookdown(data : &[[i8; MAX]; MAX], i : usize, j : usize) -> u32 {
    let mut total = 0;
    let mine = data[i][j];
    for d in i+1..MAX {
      let current = data[d][j];
      if current < mine {
          total = total + 1;
      } else {
            total = total  + 1;

        break;
      }
    }
    total

}

fn lookright(data : &[[i8; MAX]; MAX], i : usize, j : usize) -> u32 {
    let mut total = 0;
    let mine = data[i][j];
    for d in j+1..MAX {
      let current = data[i][d];
      if current < mine {
          total = total + 1;
      } else {
        total = total  + 1;
        break;
      }

    }
    total
}

fn lookleft(data : &[[i8; MAX]; MAX], i : usize, j : usize) -> u32 {
    let mut total = 0;
    let mine = data[i][j];
    for d in (0..j).rev() {        
      let current = data[i][d];
      if current < mine {
          total = total + 1;
      } else {        
        total = total  + 1;
        break;
      }

    }
    total
}


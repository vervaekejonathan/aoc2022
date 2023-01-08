use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(isize, isize);

impl Pos {
  fn successors(&self, map : &Vec<Vec<char>>) -> Vec<Pos> {
    let mut v = Vec::<Pos>::new();
    self.add_if_valid(&mut v, map, (0, 1));
    self.add_if_valid(&mut v, map, (1, 0));
    self.add_if_valid(&mut v, map, (0, -1));
    self.add_if_valid(&mut v, map, (-1, 0));
    v
  }

  fn add_if_valid( &self,
                   v : &mut Vec::<Pos>,
                   map : &Vec<Vec<char>>,
                   (offset_x, offset_y)  : (isize, isize), 
                  ) {
    let &Pos(x, y) = self;                    
    if x + offset_x >= 0 && y + offset_y >= 0 &&
       x + offset_x < map.len() as isize && y + offset_y < map[0].len() as isize {
        let current_height = map[x as usize][y as usize];
        let new_height = map[(x + offset_x) as usize][(y + offset_y) as usize];
        if new_height as u8 <= current_height as u8 + 1 {
            v.push(Pos(x + offset_x, y + offset_y));
        }
    }
  }
}

fn main() {

    let mut map = include_str!("input.txt")
    //let mut map = include_str!("test.txt")
                .split("\n")
                .map(|line| { 
                    line.chars()
                        .map(|x| x)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

    let mut start = Pos(0, 0);
    let mut goal = Pos(0, 0);
    for i in 0..map.len() {
        let line = &mut map[i];
        for j in 0..line.len() {
            if line[j] == 'S' {
                start = Pos(i as isize, j as isize);
                line[j] = 'a';
            }
            if line[j] == 'E' {
                goal = Pos(i as isize, j as isize);
                line[j] = 'z';
            }  
        }
    }

    println!("The goal is {:?}, and we start at {:?}", goal, start);
    let result = bfs(&start, 
                                       |p| p.successors(&map), 
                                       |p| *p == goal);
    println!("{}", result.expect("no path found").len() - 1);
}
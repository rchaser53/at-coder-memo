use proconio::input;
use proconio::marker::Chars;

struct Helper {
  pub max: usize,
  pub maze: Vec<Vec<char>>,
  pub col_size: usize,
  pub row_size: usize
}

impl Helper {
  pub fn new(maze: Vec<Vec<char>>) -> Helper {
    let col_size = maze[0].len() - 1;
    let row_size = maze.len() - 1;
    Helper { max: 0, maze, col_size, row_size }
  }
  
  pub fn traverse(&mut self, r: usize, c: usize, val: usize) {
    if self.maze[r][c] == '#' {
      return
    }
    self.maze[r][c] = '#';

    let mut base_stack: Vec<(usize, usize)> = vec![(r, c)];
    while 0 < base_stack.len() {
      let mut stack: Vec<(usize, usize)> = vec![];
      while 0 < base_stack.len() {
        let (r,c) = base_stack.remove(0);        
        if r < self.row_size && self.maze[r+1][c] != '#' {
          self.maze[r+1][c] = '#';
          stack.push((r+1, c));
        }
        
        if 0 < r && self.maze[r-1][c] != '#' {
          self.maze[r-1][c] = '#';
          stack.push((r-1, c));
        }
        
        if c < self.col_size && self.maze[r][c+1] != '#' {
          self.maze[r][c+1] = '#';
          stack.push((r, c+1));
        }
        
        if 0 < c && self.maze[r][c-1] != '#' {
          self.maze[r][c-1] = '#';
          stack.push((r, c-1));
        }
      }
      base_stack = stack;
      self.max += 1;
    }
    self.max -= 1;
  }
}

fn main() {
  input! {
    h: usize,
    w: usize,
    maze: [Chars;h]
  }
  
  let mut max = 0;
  for r in 0..h {
    for c in 0..w {
      let mut helper = Helper::new(maze.clone());
      helper.traverse(r, c, 0);
      max = std::cmp::max(max, helper.max);
    }
  }

  println!("{}", max);
}

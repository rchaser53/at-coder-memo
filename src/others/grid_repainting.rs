#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    h: usize,
    w: usize,
    mut maze: [Chars;h],
  }

  let DefaultValue = 3000;
  let mut rows: Vec<Vec<isize>> = vec![vec![-1;w];h];
  let mut total = 0;
  for (i, row) in maze.iter().enumerate() {
    for (ii, c) in row.iter().enumerate() {
      if c == &'.' {
        rows[i][ii] = DefaultValue;
        total += 1;
      }
    }
  }
  rows[0][0] = 1;
  
  let mut stack: VecDeque<(usize, usize)> = vec![(0,0)].into_iter().collect();
  while let Some((r, c)) = stack.pop_front() {
    let v = rows[r][c];
    maze[r][c] = '#';
    if 0 < r && maze[r-1][c] == '.' && v+1 < rows[r-1][c] {
      rows[r-1][c] = v+1;
      stack.push_back((r-1, c));
    }
    
    if 0 < c && maze[r][c-1] == '.' && v+1 < rows[r][c-1] {
      rows[r][c-1] = v+1;
      stack.push_back((r, c-1));
    }
    
    if r < h-1 && maze[r+1][c] == '.' && v+1 < rows[r+1][c] {
      rows[r+1][c] = v+1;
      stack.push_back((r+1, c));
    }
    
    if c < w-1 && maze[r][c+1] == '.' && v+1 < rows[r][c+1] {
      rows[r][c+1] = v+1;
      stack.push_back((r, c+1));
    }
  }

  if rows[h-1][w-1] == DefaultValue {
    println!("-1");
  } else {
    println!("{}", total - rows[h-1][w-1]);    
  }
}

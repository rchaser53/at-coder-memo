use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    h:usize,
    w:usize,
    mut rows:[Chars;h]
  }

  let inf = 1_000_000_000usize;
  let mut memo = vec![vec![inf;w];h];

  let mut que = VecDeque::new();
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == '#' {
        que.push_back((i,j,0));
        memo[i][j] = 0;
      }
    }
  }

  while !que.is_empty() {
    while let Some((r,c,v)) = que.pop_front() {
      let nv = v+1;
      if 0 < r {
        if memo[r-1][c] == inf {
          memo[r-1][c] = nv;
          que.push_back((r-1, c, nv));
        } else if nv < memo[r-1][c] {
          memo[r-1][c] = nv;
          que.push_front((r-1, c, nv));
        }
      }
      if r < h-1 {
        if memo[r+1][c] == inf {
          memo[r+1][c] = nv;
          que.push_back((r+1, c, nv));
        } else if nv < memo[r+1][c] {
          memo[r+1][c] = nv;
          que.push_front((r+1, c, nv));
        }
      }
      if 0 < c {
        if memo[r][c-1] == inf {
          memo[r][c-1] = nv;
          que.push_back((r, c-1, nv));
        } else if nv < memo[r][c-1] {
          memo[r][c-1] = nv;
          que.push_front((r, c-1, nv));
        }
      }
      if c < w-1 {
        if memo[r][c+1] == inf {
          memo[r][c+1] = nv;
          que.push_back((r, c+1, nv));
        } else if nv < memo[r][c+1] {
          memo[r][c+1] = nv;
          que.push_front((r, c+1, nv));
        }
      }
    }
  }

  let mut result = 0;
  for i in 0..h {
    for j in 0..w {
      result = std::cmp::max(memo[i][j], result);
    }
  }
  println!("{}", result);
}
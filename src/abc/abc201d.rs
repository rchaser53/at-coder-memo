/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(
  rows: &Vec<Vec<char>>,
  r: usize,
  c: usize
) -> isize {
  if r == 0 && c == 0 {
    return 0
  }

  if rows[r][c] == '+' {
    if (r+c) % 2 == 1 {
      1
    } else {
      -1
    }
  } else {
    if (r+c) % 2 == 1 {
      -1
    } else {
      1
    }
  }
}

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let mut memo = vec![vec![0;w];h];
  memo[h-1][w-1] = helper(&rows, h-1, w-1);

  for i in (0..w-1).rev() {
    memo[h-1][i] = memo[h-1][i+1] + helper(&rows, h-1, i);
  }

  for i in (0..h-1).rev() {
    memo[i][w-1] = memo[i+1][w-1] + helper(&rows, i, w-1);
  }

  for r in (0..h-1).rev() {
    for c in (0..w-1).rev() {
      let v = helper(&rows, r, c);
      if (r+c) % 2 == 0 {
        memo[r][c] = std::cmp::max(memo[r+1][c], memo[r][c+1]) + v;
      } else {
        memo[r][c] = std::cmp::min(memo[r+1][c], memo[r][c+1]) + v;
      }
    }
  }

  if memo[0][0] == 0 {
    println!("Draw");
  } else if memo[0][0] < 0 {
    println!("Aoki");
  } else {
    println!("Takahashi");
  }
}

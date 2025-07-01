/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h],
  }

  let inf = 1_000_000_000;
  let mut dp = vec![vec![inf;w];h];
  
  let mut stack = vec![];
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == 'E' {
        stack.push((i, j));
      }
    }
  }
  if stack.is_empty() {
    
    for rows in rows {
      println!("{}", rows.iter().collect::<String>());
    }
    return;
  }

  dp[stack[0].0][stack[0].1] = 0;
  while let Some((i, j)) = stack.pop() {
    if i > 0 && dp[i-1][j] > dp[i][j] + 1 && rows[i-1][j] != '#' {
      dp[i-1][j] = dp[i][j] + 1;
      stack.push((i-1, j));
    }
    if i < h - 1 && dp[i+1][j] > dp[i][j] + 1 && rows[i+1][j] != '#' {
      dp[i+1][j] = dp[i][j] + 1;
      stack.push((i+1, j));
    }
    if j > 0 && dp[i][j-1] > dp[i][j] + 1 && rows[i][j-1] != '#' {
      dp[i][j-1] = dp[i][j] + 1;
      stack.push((i, j-1));
    }
    if j < w - 1 && dp[i][j+1] > dp[i][j] + 1 && rows[i][j+1] != '#' {
      dp[i][j+1] = dp[i][j] + 1;
      stack.push((i, j+1));
    }
  }

  // println!("{:?}", dp);

  let mut result = rows.clone();
  for i in 0..h {
    for j in 0..w {
      if result[i][j] != '.'  {
        continue;
      }

      let mut stack = vec![(i,j)];
      while let Some((i, j)) = stack.pop() {
        if i > 0 && result[i-1][j] != '#' && dp[i-1][j] == dp[i][j] - 1 {
          result[i][j] = '^';
          stack.push((i-1, j));
        } else if i < h - 1 && result[i+1][j] != '#' && dp[i+1][j] == dp[i][j] - 1 {
          result[i][j] = 'v';
          stack.push((i+1, j));
        } else if j > 0 && result[i][j-1] != '#' && dp[i][j-1] == dp[i][j] - 1 {
          result[i][j] = '<';
          stack.push((i, j-1)); 
        } else if j < w - 1 && result[i][j+1] != '#' && dp[i][j+1] == dp[i][j] - 1 {
          result[i][j] = '>';
          stack.push((i, j+1));
        }
      }
    }
  }

  for rows in result {
    println!("{}", rows.iter().collect::<String>());
  }
}
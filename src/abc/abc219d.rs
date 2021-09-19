use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    x:usize,
    y:usize,
    vals:[(usize,usize);n]
  }

  let inf = 1_000_000;
  let mut memo = vec![vec![vec![inf;y+1];x+1];n+1];
  memo[0][0][0] = 0;

  for i in 0..n {
    memo[i+1] = memo[i].clone();
    let (a,b) = vals[i];
    for j in 0..=x {
      let ai = std::cmp::min(x, j+a);
      for k in 0..=y {
        let bi = std::cmp::min(y, k+b);
        memo[i+1][ai][bi] = std::cmp::min(memo[i+1][ai][bi], memo[i][j][k] + 1);
      }
    }
  }

  if memo[n][x][y] == inf {
    println!("-1");
  } else {
    println!("{}", memo[n][x][y]);
  }
}
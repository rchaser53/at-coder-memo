use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    vals:[usize;n]
  }

  let total = vals.iter().sum::<usize>();
  let inf = 1_000_000_000f64;
  let mut memo = vec![vec![vec![inf;total+1];total+1];n];
  memo[0][0][0] = 0f64;

  for i in 1..n {
    for j in 0..=total {
      for k in 0..=j {
        for l in 0..=total-j {
          let v = ((l as f64 - k as f64).powi(2) + 1f64).sqrt();
          let nv = memo[i-1][j][k] + v;
          if nv < memo[i][j+l][l] {
            memo[i][j+l][l] = nv;
          }
        }
      }
    }
  }
  println!("{}", memo[n-1][total][0]);
}
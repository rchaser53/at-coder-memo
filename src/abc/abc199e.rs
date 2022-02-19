/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
fn main() {
  input! {
    n:usize,
    m:usize,
    vals:[(usize,usize,usize);m]
  }

  let limit = 2usize.pow(n as u32);
  let mut dp = vec![0;limit+1];
  dp[0] = 1usize;
  let mut rules = vec![vec![];n+1];

  for (x,y,z) in vals {
    rules[x].push((y,z));
  }

  for i in 0..limit {
    let mut memo = vec![0;n+1];
    for j in 0..n {
      memo[j+1] += memo[j];
      if i >> j & 1 == 1 {
         memo[j+1] += 1;
      }
    }

    for j in 0..n {
      let v = 1 << j;
      if i & v == v { continue }
      let ni = i | v;
      let mut success = true;

      let next_group = memo[n] + 1;
      for &(y, z) in &rules[next_group] {
        let val = if (j+1) <= y {
          memo[y] + 1
        } else {
          memo[y]
        };

        if z < val {
          success = false;
          break
        }
      }

      if success {
        dp[ni] += dp[i];
      }
    }
  }
  println!("{}", dp[limit-1]);
}
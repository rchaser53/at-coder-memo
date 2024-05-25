/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn dfs(dp:&mut Vec<usize>, g:&Vec<Vec<usize>>, n:usize, cur:usize, is_tak:bool) {
  if dp[cur] != 0 {
    return
  }

  for n1 in 0..n {
    if cur >> n1 & 1 == 1 {
      continue
    }
    for &n2 in &g[n1] {
      if cur >> n2 & 1 == 1 {
        continue
      }
      let next = cur + (1<<n1) + (1<<n2);
      if dp[next] == 0 {
        dfs(dp, g, n, next, is_tak);
      }
      if dp[next] == 2 {
        dp[cur] = 1;
        return
      }
    }
  }
  dp[cur] = 2;
}

fn main() {
  input! {
    n:usize,
    ab:[(usize,usize);n],
  }

  let mut dp = vec![0;1<<n];
  let mut g = vec![vec![];n];
  for i in 0..n {
    for j in 0..i {
      if ab[i].0 == ab[j].0 || ab[i].1 == ab[j].1 {
        g[i].push(j);
        g[j].push(i);
      }
    }
  }

  dfs(&mut dp, &g, n, 0, true);
  if dp[0] == 1 {
    println!("Takahashi");
  } else {
    println!("Aoki");
  }
}
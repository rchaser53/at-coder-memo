/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

pub fn dfs(
  g: &Vec<Vec<usize>>,
  memo: &mut Vec<usize>,
  ci: usize,
  last: usize
) {
    for &ni in &g[ci] {
        if last == ni {
            continue;
        }
        dfs(g, memo, ni, ci);
        memo[ci] += memo[ni];
    }
}

pub fn main(
) {
    input! {
      n:usize,
      vals:[(Usize1,Usize1);n-1]
    }

    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        let (from, to) = vals[i];
        g[from].push(to);
        g[to].push(from);
    }

    let mut memo = vec![1; n];
    dfs(&g, &mut memo, 0, 1_000_000_000);

    let mut count = 0;
    for (l, r) in vals {
      let v = std::cmp::min(memo[r], memo[l]);
      count += v * (n - v);
    }
    println!("{}", count);
}

/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD: usize = 1_000_000_007;

fn dfs(g: &Vec<Vec<(usize, usize)>>, memo: &mut Vec<usize>, ci: usize, last: usize, v: usize) {
    memo[ci] = v;
    for &(ni, nv) in &g[ci] {
        if ni == last {
            continue;
        }
        dfs(g, memo, ni, ci, v ^ nv);
    }
}

pub fn main(
) {
    input! {
      n:usize,
      vals:[(Usize1, Usize1, usize);n-1]
    }

    let mut memo = vec![0; n];
    let mut g = vec![vec![]; n];
    for &(from, to, v) in &vals {
        g[from].push((to, v));
        g[to].push((from, v));
    }
    dfs(&g, &mut memo, 0, 1_000_000_000, 0);

    let mut result = 0;
    for i in 0..=60 {
        let base = (1 << i) % MOD;
        let mut count = 0usize;
        for &v in &memo {
            if (v >> i) & 1 == 1 {
                count += 1;
            }
        }
        if 0 < count && 0 < n - count {
            let a = count * (n - count) % MOD;
            let b = (a * base) % MOD;
            result += b;
            result %= MOD;
        }
    }

    println!("{}", result);
}

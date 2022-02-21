/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn dfs(g:&Vec<Vec<usize>>, ci:usize, last:usize, val:(usize,usize)) -> (usize,usize) {

  let mut white = val.0;
  let mut black = val.1;
  for &ni in &g[ci] {
    if ni == last { continue }

    let (w, b) = dfs(g, ni, ci, val);
    let merge = (w+b) % MOD;
    white *= merge;
    white %= MOD;
    black *= w;
    black %= MOD;
  }

  (white, black)
}

fn main() {
  input! {
    n:usize,
    edges:[(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  for (a,b) in edges {
    g[a].push(b);
    g[b].push(a);
  }
  let v = dfs(&g, 0, 1_000_000_000, (1,1));
  println!("{}", (v.0 + v.1) % MOD);
}
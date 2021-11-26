/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    l:usize,
    mut edges:[(Usize1,Usize1,usize);m],
    q:usize,
    queries:[(Usize1,Usize1);q]
  }

  let inf = 1_000_000_000_000_000usize;
  let mut g = vec![vec![inf;n];n];
  for &(f, t, c) in &edges {
    if l < c { continue }
    g[f][t] = c;
    g[t][f] = c;
  }
  for i in 0..n {
    g[i][i] = 0;
  }

  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        g[j][k] = std::cmp::min(g[j][k], g[j][i] + g[i][k]);
      }
    }
  }

  let mut ng = vec![vec![inf;n];n];
  for i in 0..n {
    ng[i][i] = 0;
    for j in 0..n {
      if i == j { continue }
      if g[i][j] <= l {
        ng[i][j] = 1;
      }
    }
  }
  
  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        ng[j][k] = std::cmp::min(ng[j][k], ng[j][i] + ng[i][k]);
      }
    }
  }

  for (from, to) in queries {
    let result = ng[from][to];
    if result >= inf {
      println!("-1");
    } else {
      println!("{}", result-1);
    }
  }
}

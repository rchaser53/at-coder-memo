/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    x:usize,
    uv:[(Usize1,Usize1);m]
  }
  
  let mut g = vec![vec![];2*n];
  for (u, v) in uv {
    g[u].push((v,1));
    g[v+n].push((u+n,1));
  }

  for i in 0..n {
    g[i].push((i+n, x));
    g[n+i].push((i, x));
  }

  let default = 10usize.pow(18);
  let mut memo = vec![default;2*n];
  memo[0] = 0;
  // BTreeSetだと間に合わない
  let mut que = BinaryHeap::new();
  que.push((Reverse(0),0));
  while let Some((Reverse(cv),ci)) = que.pop() {
    if memo[ci] < cv {
      continue
    }
    let m = g[ci].len();
    for i in 0..m {
      let (nj,av) = g[ci][i];
      let nv = cv + av;
      if memo[nj] > nv {
        memo[nj] = nv;
        que.push((Reverse(nv),nj));
      }
    }
  }
  println!("{}", memo[n-1].min(memo[2*n-1]));
}
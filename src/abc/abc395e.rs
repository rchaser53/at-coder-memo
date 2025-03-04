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
  let mut stack = vec![(0,0)];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      for &(ni,av) in g[ci].iter() {
        let nv = cv+av;
        if memo[ni] > nv {
          memo[ni] = nv;
          new_stack.push((ni,nv));
        }
      }
    }
    stack = new_stack;
  }
  println!("{}", memo[n-1].min(memo[2*n-1]));
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;n],
    uvb:[(Usize1,Usize1,usize);m]
  }

  let inf = 10usize.pow(18);
  let mut g = vec![HashMap::new();n];
  for (u,v,b) in uvb {
    let entry = g[u].entry(v).or_insert(inf);
    if b < *entry {
      *entry = b;
    }
    let entry = g[v].entry(u).or_insert(inf);
    if b < *entry {
      *entry = b;
    }
  }
  let g = g.into_iter().map(|v| v.into_iter().collect::<Vec<_>>()).collect::<Vec<Vec<(usize,usize)>>>();
  let mut memo = vec![inf;n];
  memo[0] = a[0];

  let mut stack = vec![(0,a[0])];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      for &(ni, av) in &g[ci] {
        let nv = a[ni] + av + cv;
        if nv < memo[ni] {
          memo[ni] = nv;
          new_stack.push((ni,nv));
        }
      }
    }
    stack = new_stack;
  }

  memo.remove(0);
  println!("{}", memo.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
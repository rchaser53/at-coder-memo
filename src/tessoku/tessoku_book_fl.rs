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
    edges:[(Usize1,Usize1,usize);m]
  }
  let mut g = vec![vec![];n];
  for (a,b,c) in edges {
    g[a].push((b,c));
    g[b].push((a,c));
  }

  let mut stack = vec![(0,0)];
  let inf = 1_000_000_000_000_000usize;
  let mut memo = vec![(inf, HashSet::new());n];
  memo[0] = (0, HashSet::new());
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      for &(ni, v) in &g[ci] {
        let nv = v + cv;
        if nv < memo[ni].0 {
          let mut new_set = HashSet::new();
          new_set.insert(ci);
          memo[ni] = (nv, new_set);
          new_stack.push((ni, nv));
        } else if nv == memo[ni].0 {
          memo[ni].1.insert(ci);
        }
      }
    } 
    stack = new_stack;
  }

  let mut set = HashSet::new();
  let mut stack = vec![n-1];
  set.insert(n-1);
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some(ci) = stack.pop() {
      for &ni in &memo[ci].1 {
        if !set.contains(&ni) {
          set.insert(ni);
          new_stack.push(ni);
        }
      }
    }
    stack = new_stack;
  }
  println!("{}", set.len());
}
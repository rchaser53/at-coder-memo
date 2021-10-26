/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    x:usize,
    edges:[(Usize1,Usize1,usize);n-1]
  }

  let mut g = vec![vec![];n];
  for (a, b, c) in edges {
    g[a].push((b, c));
    g[b].push((a, c));
  }

  for i in 0..n {
    let mut stack = vec![(i, 1_000_0000, 0)];
    while !stack.is_empty() {
      let mut new_stack = vec![];
      while let Some((ci, last, val)) = stack.pop() {
        if val == x {
          println!("Yes");
          return
        }
        
        for &(ni, nv) in &g[ci] {
          if last == ni { continue }
          new_stack.push((ni, ci, nv + val));
        }
      }
      stack = new_stack;
    }
  }

  println!("No");
}
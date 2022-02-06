/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    edges:[(Usize1,usize);q]
  }

  let mut g = vec![vec![];n+1];
  let mut is_zero = false;
  for (a, b) in edges {
    if a == 0 || b == 0 {
      is_zero = true;
    }
    g[a].push(b);
    g[b].push(a);
  }

  if !is_zero {
    println!("No");
    return
  }

  let mut seen = vec![false;n+1];
  seen[0] = true;
  let mut stack = vec![0];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some(ci) = stack.pop() {
      for &ni in &g[ci] {
        if seen[ni] { continue }
        seen[ni] = true;
        new_stack.push(ni);
      }
    }
    stack = new_stack;
  }

  if seen[n] {
    println!("Yes");
  } else {
    println!("No");
  }
}
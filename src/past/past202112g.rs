/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn check(g:&Vec<HashSet<usize>>, u:usize, v:usize) -> bool {
  let n = g.len();
  let mut memo = vec![false;n];
  let mut stack = vec![u];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some(ci) = stack.pop() {
      if memo[ci] { continue }
      memo[ci] = true;
      for &ni in &g[ci] {
        if memo[ni] { continue }
        new_stack.push(ni);
      }
    }
    stack = new_stack;
  }
  memo[v]
}

fn main() {
  input! {
    n:usize,
    q:usize
  }

  let mut g = vec![HashSet::new();n];
  for _ in 0..q {
    input!{
      t:usize,
      u:Usize1,
      v:Usize1
    }

    if t == 2 {
      if check(&g, u, v) {
        println!("Yes");
      } else {
        println!("No");
      }
      continue
    }

    if g[u].contains(&v) {
      g[u].remove(&v);
      g[v].remove(&u);
    } else {
      g[u].insert(v);
      g[v].insert(u);
    }
  }
}
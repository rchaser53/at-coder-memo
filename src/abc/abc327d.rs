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
    a:[Usize1;m],
    b:[Usize1;m]
  }

  let mut memo = vec![3;n];
  let mut g = vec![HashSet::new();n];
  for i in 0..m {
    g[a[i]].insert(b[i]);
    g[b[i]].insert(a[i]);
  }

  for i in 0..m {
    let ti = a[i];
    if memo[ti] != 3 { continue }
    let mut stack = vec![(ti,1)];
    memo[ti] = 1;

    while !stack.is_empty() {
      let mut new_stack = vec![];
      while let Some((ci, v1)) = stack.pop() {
        let v2 = v1 ^ 1;

        for &ni in &g[ci] {
          if memo[ni] == 3 {
            memo[ni] = v2;
            new_stack.push((ni, v2));
          } else if memo[ni] == v1 {
            println!("No");
            return
          }
        }
      }
      stack = new_stack;
    }
  }

  println!("Yes");
}
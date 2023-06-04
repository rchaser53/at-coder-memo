/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    d:isize,
    xy:[(isize,isize);n]
  }

  let mut g = vec![vec![];n];
  let dd = d*d;

  for i in 0..n {
    let (x1, y1) = xy[i];
    for j in 0..n {
      if i == j { continue }
      let (x2,y2) = xy[j];

      if (x1-x2).pow(2) + (y1-y2).pow(2) <= dd {
        g[i].push(j);
      }
    }
  }

  let mut memo = vec![false;n];
  memo[0] = true;

  let mut stack = vec![0];

  while !stack.is_empty() {
    let mut new_stack = vec![];
    for ci in stack {
      for &ni in &g[ci] {
        if !memo[ni] {
          memo[ni] = true;
          new_stack.push(ni);
        }
      }
    }
    stack = new_stack;
  }

  for v in memo {
    if v {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}
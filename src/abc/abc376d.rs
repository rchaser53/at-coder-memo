/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;


fn main() {
  input! {
    n:usize,
    m:usize,
    ab:[(Usize1,Usize1);m]
  }

  let mut g = vec![vec![];n];
  for (a,b) in ab {
    g[a].push(b);
  }
  let default = 1_000_000_000;
  let mut memo = vec![default;n];

  let mut stack = vec![(0,0)];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      let nv = cv + 1;
      for &ni in &g[ci] {
        if nv < memo[ni] {
          memo[ni] = nv;
          new_stack.push((ni,nv));
        }
      }
    }
    stack = new_stack;
  }

  if memo[0] == default {
    println!("-1");
  } else {
    println!("{}", memo[0]);
  }
}
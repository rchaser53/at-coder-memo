/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:usize,
    b:usize,
    c:usize,
    d:[[usize;n];n]
  }

  let inf = 10usize.pow(15);
  let mut memo = vec![inf;n];
  memo[0] = 0;

  let mut stack = vec![(0usize,0usize)];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      for ni in 0..n {
        if ci == ni { continue }
        let nv = d[ci][ni] * a + cv;
        if nv < memo[ni] {
          memo[ni] = nv;
          new_stack.push((ni, nv));
        }
      }
    }
    stack = new_stack;
  }

  let mut stack = vec![(n-1,0usize)];
  let mut train_memo = vec![inf;n];
  train_memo[n-1] = 0;
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      for ni in 0..n {
        if ci == ni { continue }
        let nv = d[ci][ni] * b + c + cv;
        if nv < train_memo[ni] {
          train_memo[ni] = nv;
          new_stack.push((ni, nv));
        }
      }
    }
    stack = new_stack;
  }

  let mut result = inf;
  for i in 0..n {
    let cv = memo[i];
    let av = train_memo[i];
    result = result.min(cv+av);
  }
  println!("{}", result.min(memo[n-1]));
}
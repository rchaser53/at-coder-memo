/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }

  let mut g = vec![vec![0;n];n];
  for i in 0..n-1 {
    input! {
      arr:[usize;n-1-i]
    }
    for j in 0..arr.len() {
      g[i][j+i+1] = arr[j];
      g[j+1+i][i] = arr[j];
    }
  }

  let mut stack = vec![(0,0)];
  let limit = 1 << n;
  let mut memo = vec![0;limit];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci, cv)) = stack.pop() {
      for i in 0..n {
        let v1 = 1 << i;
        if ci & v1 == v1 { continue }
        for j in 0..n {
          let v2 = 1 << j;
          if i == j || ci & v2 == v2 { continue }
          
          let ni = (ci | v1) | v2;
          let nv = cv + g[i.max(j)][i.min(j)];
          if memo[ni] < nv {
            memo[ni] = nv;
            new_stack.push((ni,nv));
          } 
        }
      }
    }
    stack = new_stack;
  }
  let mut result = 0usize;

  for i in 0..limit {
    result = result.max(memo[i]);
  }
  println!("{}", result);
}
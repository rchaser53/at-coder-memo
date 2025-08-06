/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    abw:[(Usize1,Usize1,usize);m]
  }

  let max = 1 << 10 + 10;
  let mut memo = vec![vec![false;max];n];
  memo[0][0] = true;
  let mut stack = vec![(0,0)];

  let mut g = vec![vec![];n];
  for (a,b,w) in abw {
    g[a].push((b,w));
  }

  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci, cv)) = stack.pop() {
      for &(ni, nv) in &g[ci] {
        let nmask = cv ^ nv;
        if !memo[ni][nmask] {
          memo[ni][nmask] = true;
          new_stack.push((ni, nmask));
        }
      }
    }
    stack = new_stack;
  }

  for i in 0..max {
    if memo[n-1][i] {
      println!("{}", i);
      return;
    }
  }
  println!("-1");
}
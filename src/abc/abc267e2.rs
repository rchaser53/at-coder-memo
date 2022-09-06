/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    m:usize,
    vals:[usize;n],
    edges:[(Usize1,Usize1);m]
  }

  let mut connects = vec![vec![];n];
  let mut memo = vec![0;n];
  for &(a, b) in &edges {
    memo[a] += vals[b];
    memo[b] += vals[a];
    connects[a].push((b,vals[a]));
    connects[b].push((a,vals[b]));
  }

  let mut left = *memo.iter().min().unwrap();
  let mut right = *memo.iter().max().unwrap()+1;
  let mut temps = vec![0;n];
  while left  < right {
    let base_val = (left+right)/2;
    for i in 0..n {
      temps[i] = memo[i];
    }

    let mut s = vec![];
    let mut score = 0;
    let mut seen = vec![false;n];
    for i in 0..n {
      if memo[i] <= base_val {
        s.push(i);
        seen[i] = true;
      }
    }
    
    while let Some(i) = s.pop() {
      score += 1;
      for &(ti, v) in &connects[i] {
        temps[ti] -= v;
        if temps[ti] <= base_val && !seen[ti] {
          seen[ti] = true;
          s.push(ti);
        }
      }
    }
    
    if score == n {
      right = base_val;
    } else {
      left = base_val+1;
    }
  }
  println!("{}", left);
}
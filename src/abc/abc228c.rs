/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    k:Usize1,
    vals:[[usize;3];n]
  }

  let mut memo = vec![(0,0);n];
  for i in 0..n {
    memo[i] = (vals[i].iter().sum::<usize>(), i);
  }
  memo.sort();
  memo.reverse();

  let mut result = vec![false;n];
  

  for i in 0..=k {
    result[memo[i].1] = true;
  }
  let limit = memo[k].0;

  for i in k+1..n {
    if memo[i].0 + 300 < limit {
      break
    }
    result[memo[i].1] = true;
  }

  for v in result {
    if v {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}
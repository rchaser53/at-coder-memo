/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:[Chars;n]
  }
  
  let mut bset = BTreeSet::new();
  for i in 0..n {
    bset.insert((s[i].len(), i));
  }

  let max_len = bset.iter().rev().next().unwrap().0;
  let mut memo = vec![vec![];max_len];

  for i in (0..n).rev() {
    let m = bset.iter().rev().next().unwrap().0;
    let x = s[i].len();
    for j in 0..m {
      if x <= j {
        memo[j].push('*');
      } else {
        memo[j].push(s[i][j]);
      }
    }

    bset.remove(&(x,i));
  }

  for s in memo {
    println!("{}", s.into_iter().map(|v| v.to_string()).collect::<String>());
  }
}
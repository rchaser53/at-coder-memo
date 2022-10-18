/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:usize,
    b:usize,
    wv:[(usize,isize);n]
  }

  let inf = -10isize.pow(12);
  let mut memo = vec![vec![inf;b+1];a+1];
  memo[0][0] = 0;
  for (w,v) in wv {
    let mut new_memo = memo.clone();
    for i in 0..=a {
      for j in 0..=b {
        if w <= i && w <= j {
          new_memo[i][j] = new_memo[i][j].max(memo[i-w][j] + v).max(memo[i][j-w]+v);
        } else if w <= i {
          new_memo[i][j] = new_memo[i][j].max(memo[i-w][j] + v);
        } else if w <= j {
          new_memo[i][j] = new_memo[i][j].max(memo[i][j-w]+v);
        }
      }
    }
    memo = new_memo;
  }

  let mut result = 0;
  for i in 0..=a {
    for j in 0..=b {
      result = result.max(memo[i][j]);
    }
  }
  println!("{}", result);
}
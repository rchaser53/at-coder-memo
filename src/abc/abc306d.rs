/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    xy:[(isize,isize);n]
  }

  let mut memo = (0,0);
  for (x,y) in xy {
    let mut new_memo = memo.clone();
    if x == 0 {
      new_memo.0 = memo.0.max(memo.0+y).max(memo.1+y);
    } else {
      new_memo.1 = memo.1.max(memo.0+y);
    }
    memo = new_memo;
  }
  println!("{}", memo.0.max(memo.1));
}
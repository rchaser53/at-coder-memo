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
    lr:[(Usize1,Usize1);m]
  }

  let mut memo = vec![(0,0);n];
  for (l,r) in lr {
    memo[l].0 += 1;
    memo[r].1 += 1;
  }

  let mut result = n+1;
  let mut temp = 0;
  for (add,minus) in memo {
    temp += add;
    result = std::cmp::min(result, temp);
    temp -= minus;
  }

  println!("{}", result);
}
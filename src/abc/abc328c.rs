/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    s:Chars,
    lr:[(Usize1,Usize1);q]
  }

  let mut memo = vec![0;n+1];
  for i in 1..n {
    memo[i+1] = memo[i];
    if s[i] == s[i-1] {
      memo[i+1] += 1;
    }
  }

  for (l,r) in lr {
    println!("{}", memo[r+1] - memo[l+1]);
  }
}
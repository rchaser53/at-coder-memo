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
    xy:[(usize,usize);q]
  }

  let mut memo = vec![1;n];
  let mut o = 0;
  for (x,y) in xy {
    let mut count = 0;
    while o <= x-1 {
      memo[y-1] += memo[o];
      count += memo[o];
      memo[o] = 0;
      o += 1;
    }
    println!("{}", count);
  }
}
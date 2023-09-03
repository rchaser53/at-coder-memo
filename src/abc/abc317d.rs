/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    xyz:[(isize,isize,usize);n]
  }

  let default_value = 10usize.pow(16);
  let max = 10usize.pow(5)+10;
  let mut memo = vec![default_value;10usize.pow(5)+10];
  memo[0] = 0;

  let mut tot = 0;
  for i in 0..n {
    let (x,y,z) = xyz[i];
    tot += z;
    let mut new_memo = memo.clone();
    let hv = (x+y)/2+1;
    let need = (hv-x).max(0) as usize;

    for j in 0..max {
      let nj = (j+z).min(max-1);
      new_memo[nj] = new_memo[nj].min(memo[j]+need);
    }
    memo = new_memo;
  }
  let need = tot / 2 + 1;
  let mut result = default_value;
  for i in need..max {
    result = result.min(memo[i]);
  }
  println!("{}", result);
}
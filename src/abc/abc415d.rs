/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    mut n:usize,
    m:usize,
    ab:[(usize, usize);m],
  }

  let mut que = Vec::new();
  for (a,b) in ab {
    que.push((a-b, a, b));
  }

  que.sort();
  let mut count = 0;
  for (d,a,_b) in que {
    if a > n { continue }
    let x = (n-a) / d + 1;
    count += x;
    n -= x * d;
  }
  
  println!("{}", count);
}
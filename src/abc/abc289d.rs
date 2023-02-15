/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    a:[usize;n],
    m:usize,
    b:[usize;m],
    x:usize
  }

  let limit = 10usize.pow(5)+10;
  let mut memo = vec![false;limit+1];
  memo[0] = true;
  for i in b {
    memo[i] = true;
  }

  let mut stack = vec![0];
  while let Some(ci) = stack.pop() {
    for &ai in &a {
      let ni = ai + ci;
      if limit <= ni { continue }
      if memo[ni] { continue }
      memo[ni] = true;
      stack.push(ni);
    }
  }

  if memo[x] {
    println!("Yes");
  } else {
    println!("No");
  }
}
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    q:usize,
    h:usize,
    s:usize,
    d:usize,
    n:usize
  }

  let mut result = 0usize;
  let ev = n / 2;
  let mut arr = vec![q*8,h*4,s*2,d];
  arr.sort();
  result += ev * arr[0];

  if n % 2 == 1 {
    let mut arr = vec![q*4,h*2,s];
    arr.sort();
    result += arr[0];
  }
  println!("{}", result);
}
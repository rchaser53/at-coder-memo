#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
 
fn bs(
  arr: &Vec<usize>,
  v: usize,
  l: usize,
  r: usize
) -> usize {
  if r <= l {
    return if arr[r] < v {
      arr.len()
    } else {
      l
    }
  }

  let middle = (l + r) / 2;
  if arr[middle] < v {
    bs(arr, v, middle + 1, r)
  } else {
    bs(arr, v, l, middle)
  }
}

fn main() {
  input!{
    n: usize,
    mut a_vals: [usize;n],
    mut b_vals: [usize;n],
    mut c_vals: [usize;n],
  }
  a_vals.sort();
  b_vals.sort();
  c_vals.sort();

  let mut bc = vec![0;n];
  for i in 0..n {
    bc[i] = n - bs(&c_vals, b_vals[i]+1, 0, n-1);
  }
  
  for i in (0..n-1).rev() {
    bc[i] += bc[i+1];
  }

  let mut result = 0;
  for i in 0..n {
    let ti = bs(&b_vals, a_vals[i]+1, 0, n-1);
    if ti < n {
      result += bc[ti];    
    }
  }
  
  println!("{}", result); 
}

/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[Usize1;n]
  }
  
  let mut memo = vec![0;n];
  let mut loop_index = vec![-1;n];
  let mut result = 0;
  for i in 0..n {
    if loop_index[i] != -1 {
      continue
    }
    let mut v = i;
    memo[i] = 1;
    loop_index[i] = i as i32;
    loop {
      let nv = a[v];
      if memo[nv] != 0 {
        if loop_index[nv] == i as i32 {
          result += memo[v] + 1 - memo[nv];
        }
        break
      }
      loop_index[nv] = i as i32;
      memo[nv] = memo[v] + 1;
      v = nv;
    }
  }
 
  println!("{}", result);
}
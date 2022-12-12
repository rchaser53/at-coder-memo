/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn dfs(arr: &Vec<usize>, i:usize) -> usize {
  if i == 0 { return 0 }
  let ni = i - 1;

  let n = arr.len();
  let mut unused = vec![];
  let mut used = vec![];
  for j in 0..n {
    if arr[j] >> ni & 1 == 1 {
      used.push(arr[j]);
    } else {
      unused.push(arr[j]);
    }
  }

  if used.is_empty() {
    dfs(&unused, ni) 
  } else if unused.is_empty() {
    dfs(&used, ni)
  } else {
    dfs(&used, ni).min(dfs(&unused, ni)) | (1usize << ni)
  }
}

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }
  println!("{}", dfs(&a, 30));
}
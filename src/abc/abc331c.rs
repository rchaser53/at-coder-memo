/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }
  
  let mut map = HashMap::new();
  for i in 0..n {
    map.entry(a[i]).or_insert(vec![]).push(i);
  }

  let mut arrs = map.into_iter().collect::<Vec<(usize,Vec<usize>)>>();
  arrs.sort();

  let m = arrs.len();
  let mut tot = 0;
  let mut result = vec![0;n];
  for i in (0..m).rev() {
    let (val, arr) = &arrs[i];
    for j in arr {
      result[*j] = tot;
    }
    tot += val * arr.len();
  }
  
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
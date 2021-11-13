/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:usize,
    mut xy:[(Usize1,Usize1);n]
  }
  let mut idx = vec![0;n];
  for i in 0..n {
    idx[xy[i].0] = i;
  }
  let mut col = vec![0;n];
  let mut siz = vec![0;n];
  let mut count = 0;
  let mut min = n+1;
  for i in 0..n {
    let (_, y) = xy[idx[i]];
    if min == n - i {
      count += 1;
    }
    min = std::cmp::min(min, y);
    col[idx[i]] = count;
    siz[count] += 1;
  }
  for i in 0..n {
    println!("{}", siz[col[i]]);
  }
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  let mut ml = vec![0;n+1];
  let mut mr = vec![0;n+1];
  let mut set = HashSet::new();
  for i in 0..n {
    set.insert(a[i]);
    ml[i] = set.len();
  }

  set = HashSet::new();
  for i in (0..n).rev() { 
    set.insert(a[i]);
    mr[i] = set.len();
  }

  let mut result = ml[n-1];
  for i in 0..n {
    result = result.max(ml[i]+mr[i+1]);
  }
  println!("{result}");
}
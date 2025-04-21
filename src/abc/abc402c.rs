/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
  }

  let mut memo = vec![HashSet::new();m];
  let mut map = HashMap::new();
  for i in 0..m {
    input! {
      k:usize,
      a:[usize;k]
    }

    for j in 0..k {
      let aj = a[j];
      map.entry(aj).or_insert(vec![]).push(i);
      memo[i].insert(aj);
    }
  }

  let mut result = 0;
  input! {
    b:[usize;n]
  }

  for i in b {
    if let Some(arr) = map.remove(&i) {
      for j in arr {
        memo[j].remove(&i);
        if memo[j].is_empty() {
          result += 1;
        }
      }
    }
    println!("{}", result);
  } 
}
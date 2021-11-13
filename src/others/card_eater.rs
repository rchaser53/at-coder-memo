#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    mut vals: [usize;n]
  }
  
  let mut memo: HashMap<usize,usize> = HashMap::new();
  for v in vals {
    let entry = memo.entry(v).or_insert(0);
    *entry += 1;
  }
 
  let mut count = 0;
  let len = memo.keys().len();
  for (_, val) in memo {
    if val % 2 == 0 {
      count += 1;      
    }
  }
  
  if count % 2 == 1 {
    println!("{}", len - 1);
  } else {
    println!("{}", len);
  }
}
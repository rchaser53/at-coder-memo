#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: u128,
  }

  let mut key = 1;
  let mut map = HashMap::new();
  while 5u128.pow(key) <= n {
    map.insert(key, 5u128.pow(key));
    key += 1;
  }
  
  let mut b = 1;
  while 3u128.pow(b) <= n {
    let v = 3u128.pow(b);
    for (key, val) in map.iter() {
      if val + v == n {
        println!("{} {}", b, key);
        return
      }
    }
    b += 1;
  }
  
  println!("-1");
}
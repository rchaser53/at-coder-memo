/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    mut k:usize
  }
 
  let mut i = 2;
  let mut map = HashMap::new();
  while i * i <= k {
    if k % i == 0 {
      k /= i;
      *map.entry(i).or_insert(0) += 1;
    } else {
      i += 1;
    }
  }
 
  if k > 1 {
    *map.entry(k).or_insert(0) += 1;
  }
 
  let mut max = 2;
  for (key, mut num) in map {
    if key == 1 { continue }
    
    let mut i = 1;
    while 0 < num {
      let mut temp = i * key;
      while temp % key == 0  {
        temp /= key;
        num -= 1;
      }
      i += 1;
    }
    max = max.max(key*(i-1));
  }
 
  println!("{}", max);
}
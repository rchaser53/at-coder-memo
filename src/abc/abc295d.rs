/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    s:Chars
  }
    
  let mut map = HashMap::new();
  let s = s.into_iter().map(|v| (v as u8 - '0' as u8) as usize).collect::<Vec<usize>>();
  let mut temp = vec![false;10];
  map.insert(0, 1);
  for i in s {
    temp[i] = !temp[i];
    let mut val = 0;
    for j in 0..10 {
      if temp[j] {
        val += 1 << j;
      }
    }
    *map.entry(val).or_insert(0) += 1;
  }

  let mut result = 0usize;
  for (_, num) in map {
    result += num * (num-1) / 2;
  }
  println!("{}", result);
}
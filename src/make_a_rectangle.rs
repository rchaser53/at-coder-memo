#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut map: HashMap<usize, usize> = HashMap::new();
  for v in vals {
    let entry = map.entry(v).or_insert(0);
    *entry += 1;
  }
  
  let mut first = 0;
  let mut second = 0;
  for (v, count) in map {
    if count > 3 {
      if first <= v {
        second = v;
        first = v;
      } else if second <= v {
        second = v;
      }
    } else if count > 1 {
      if first <= v {
        second = first;
        first = v;
      } else if second <= v {
        second = v;
      }    
    }
  }
  println!("{}", first * second);
}

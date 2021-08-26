/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    mut vals:[usize;n]
  }
  vals.sort();
  vals.reverse();
  let mut map = HashMap::new();
  for &v in &vals {
    *map.entry(v).or_insert(0) += 1;
  }

  let mut result = 0;
  for v in vals {
    let num = if let Some(num) = map.get(&v) {
      num - 1
    } else {
      continue
    };

    if num == 0 {
      map.remove(&v);
    } else {
      map.insert(v, num);
    }

    for ti in 0..=60 {
      let tv = 2usize.pow(ti);
      if tv <= v { continue }
      let diff = tv - v;
      let num = if let Some(num) = map.get(&diff) {
        num - 1
      } else {
        continue
      };

      result += 1;
      if num == 0 {
        map.remove(&diff);
      } else {
        map.insert(diff, num);
      }
      break
    }
  }
  println!("{}", result);
}

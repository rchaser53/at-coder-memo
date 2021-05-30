/* OUTPUT FILE */
#![allow(unused_imports)]
use permutohedron::heap_recursive;
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn culc(
  mut n:usize
) -> HashMap<usize,usize> {
  let mut i = 2;
  let mut map = HashMap::new();
  let base = n;
  while i*i <= base {
    if n % i == 0 {
      *map.entry(i).or_insert(0) += 1;
      n /= i;
    } else {
      i += 1;
    }
  }
  if 1 < n {
    *map.entry(i).or_insert(0) += 1;
  }
  map
}

pub fn main(
) {
    input! {
      n:usize,
    }

    let map = culc(n);
    let mut result = 0;
    for (_, mut v) in map {
      let mut temp = 1;
      while temp <= v {
        result += 1;
        v -= temp;
        temp += 1;
      }
    }
    println!("{}", result);
}

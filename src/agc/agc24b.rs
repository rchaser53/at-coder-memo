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
      vals:[usize;n],
    }

    let mut map = HashMap::new();
    for v in vals {
      let nv = if let Some(rv) = map.remove(&(v-1)) {
        rv + 1
      } else {
        1
      };
      map.insert(v, nv);
    }

    let mut max = 0;
    for (_, v) in map {
      max = std::cmp::max(max, v);
    }
    println!("{}", n - max);
}

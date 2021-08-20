/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }
  
  let mut map = HashMap::new();
  let mut stack = vec![1usize];
  let mut last = 1_000_000_000_000;
  for v in vals {
    if last == v {
      continue
    }

    let top_val = stack[stack.len()-1];
    if let Some(mv) = map.get_mut(&v) {
      let nv = (top_val + *mv) % MOD;
      stack.push(nv);
      *mv = nv;
    } else {
      map.insert(v, top_val);
      stack.push(top_val);
    }
    last = v;
  }

  println!("{}", stack[stack.len()-1]);
}

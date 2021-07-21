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
    n: usize,
    mut a: [usize;n],
    mut b: [usize;n],
    mut c: [usize;n],
  }

  let inf = 1_000_000_000_000usize;
  a.sort();
  b.sort();
  c.sort();

  let mut ai = 0;
  let mut bi = 0;
  let mut ci = 0;
  let mut result = 0;
  while ai < n && bi < n && ci < n {
    let av = a[ai];

    let mut bv = inf;
    for i in bi..n {
      if av < b[i] {
        bi = i + 1;
        bv = b[i];
        break
      }
    }

    if bv == inf { break }

    let mut cv = inf;
    for i in ci..n {
      if bv < c[i] {
        ci = i + 1;
        cv = c[i];
        break
      }
    }
    if cv == inf { break }
    result += 1;
    ai += 1;
  }
  
  println!("{}", result);
}

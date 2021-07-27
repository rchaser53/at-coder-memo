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
    mut a:[usize;n],
    b:[usize;n]
  }

  let mut set = HashSet::new();
  a.sort();
  for i in 0..n {
    let bv = b[i];
    set.insert(a[0] ^ bv);
  }

  let mut result = vec![];
  for v in set {
    let mut created_vals = vec![0;n];
    for i in 0..n {
      created_vals[i] = b[i] ^ v;
    }
    created_vals.sort();

    let mut success = true;
    for i in 0..n {
      if created_vals[i] != a[i] {
        success = false;
        break
      }
    }
    if success {
      result.push(v);
    }
  }

  result.sort();
  println!("{}", result.len());
  for v in result {
    println!("{}", v);
  }
}

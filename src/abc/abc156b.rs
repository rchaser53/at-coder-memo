/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use permutohedron::heap_recursive;
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      k:usize,
    }

    if n < k {
      println!("1");
      return
    }

    let mut i = 1;
    while k.pow(i) <= n {
      i += 1;
    }
    println!("{}", i);
}

/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use permutohedron::heap_recursive;
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
    input! {
      n:usize,
      d:usize
    }

    let v = 1 + 2 * d;
    let mut result = n / v;
    if n % v != 0 {
      result += 1;
    }
    println!("{}", result);
}

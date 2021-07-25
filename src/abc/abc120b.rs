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
      mut a:usize,
      mut b:usize,
      k:usize
    }

    if b < a {
      std::mem::swap(&mut b, &mut a);
    }    
    let mut stack = vec![];
    for i in 1..=a {
      if a % i == 0 && b % i == 0 {
        stack.push(i)
      }
    }

    stack.sort();
    stack.reverse();
    println!("{}", stack[k-1]);
}

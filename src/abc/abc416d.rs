/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    t:usize
  }

  for _ in 0..t {
    input! {
      n:usize,
      m:usize,
      mut a:[usize;n],
      mut b:[usize;n]
    }

    a.sort();
    b.sort();
    b.reverse();
    let mut result = a.iter().sum::<usize>() + b.iter().sum::<usize>();
    let mut i = 0;
    for ai in a {
      if i < b.len() && m <= ai + b[i] {
        result -= m;
        i += 1;
      }
    }
    println!("{}", result);
  }
}
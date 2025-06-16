/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
  }

  let mut stack = (1..=n).collect::<Vec<_>>();
  let mut slide = 0;
  for _ in 0..q {
    input! {
      t: usize,
    }

    if t == 1 {
      input! {
        p: Usize1,
        x: usize
      }
      let ti = (p+n-slide) % n;
      stack[ti] = x;
    } else if t == 2 {
      input! {
        p: Usize1,
      }
      println!("{}", stack[(p + n-slide) % n]);
    } else {
      input! {
        k: usize
      }
      slide = (slide + n - (k%n)) % n;
    }
  }
}
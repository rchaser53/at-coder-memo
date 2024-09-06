/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    h:[isize;n]
  }

  let mut t = 0;
  for v in h {
    let num = v / 5;
    t += num * 3;

    let mut rv = v % 5;
    while 0 < rv {
      t += 1;
      if t % 3 == 0 {
        rv -= 3;
      } else {
        rv -= 1;
      }
    }
  }

  println!("{}", t);

}
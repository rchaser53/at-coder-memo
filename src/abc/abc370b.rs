/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize
  }

  let mut arr = vec![];
  for i in 0..n {
    input! {
      a: [Usize1;i+1]
    }
    arr.push(a);
  }

  let mut now = 0;
  for i in 0..n {
    let (ri, ci) = if now >= i {
      (now, i)
    } else {
      (i, now)
    };
    now = arr[ri][ci];
  }

  println!("{}", now+1);
}
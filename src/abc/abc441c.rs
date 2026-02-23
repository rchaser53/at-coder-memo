/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    mut x:isize,
    mut a:[isize;n]
  }

  a.sort();
  let mut a = a.into_iter().take(k).collect::<Vec<_>>();
  a.reverse();

  // dbg!(&a);
  let mut count = n-k;
  for v in a {
    x -= v;
    count += 1;
    if x <= 0 {
      break;  
    }
  }

  if x > 0 {
    println!("-1");
  } else {
    println!("{}", count);
  }
}
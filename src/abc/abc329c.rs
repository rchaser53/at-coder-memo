/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut set = HashSet::new();
  let mut x = '1';
  let mut count = 0;
  for c in s {
    if count == 0 {
      count += 1;
      x = c;
    } else {
      if c != x {
        x = c;
        count = 1;
      } else {
        count += 1;
      }
    }
    set.insert((x, count));
  }

  println!("{}", set.len());
}
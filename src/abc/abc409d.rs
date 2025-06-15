/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    t:usize,
  }

  // cba => bac
  // cbd => bdc x => bcd
  for _ in 0..t {
    input! {
      n:usize,
      mut s:Chars,
    }
    let mut ci = 0;
    let mut shoud_change = false;
    for i in 1..n {
      if shoud_change {
        if s[ci] < s[i] {
          break
        }
        s.swap(ci, i);
        ci = i;
      } else {
        if s[ci] > s[i] {
          shoud_change = true;
          s.swap(ci, i);
        }
        ci = i;
      }
    }

    println!("{}", s.iter().collect::<String>());
  }
}
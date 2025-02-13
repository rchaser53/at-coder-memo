/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    w:usize,
    b:usize,
  }

  let s = "wbwbwwbwbwbw".to_string().chars().collect::<Vec<char>>();
  let n = s.len();
  let tot = w+b;

  for i in 0..=2*n {
    let mut wc = 0;
    let mut bc = 0;

    let mut ci = i;
    while wc + bc < tot {
      if s[ci % n] == 'w' {
        wc += 1;
      } else {
        bc += 1;
      }
      ci += 1;
    }

    if wc == w && bc == b {
      println!("Yes");
      return
    }
  }
  println!("No");
}
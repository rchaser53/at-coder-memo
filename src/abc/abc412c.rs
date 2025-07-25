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
    helper();
  }
}

fn helper() {
    let mut p = 0;
    let mut result = 1;

    input! {
      n:usize,
      s:[usize;n]
    }

    let mut success = true;
    loop {
      if 2 * s[p] >= s[n-1] {
        result += 1;
        break
      }

      match s
      .iter()
      .enumerate()
      .filter(|&(_,&x)| x > s[p] && x <= 2 * s[p])
      .max_by_key(|&(_,&x)| x) {
        Some((i, _)) => {
          p = i;
          result += 1;
        }
        None => {
          println!("-1");
          success = false;
          break
        }
      }
    }

    if success {
      println!("{}", result);
    }
}
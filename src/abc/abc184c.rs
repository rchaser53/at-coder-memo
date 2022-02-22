/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

fn main() {
  input! {
    r1:isize,
    c1:isize,
    r2:isize,
    c2:isize,
  }

  if r1 == r2 && c1 == c2 {
    println!("0");
    return
  }

  if r1 + c1 == r2 + c2 ||
    r1 - c1 == r2 - c2 || 
    (r1-r2).abs() + (c1-c2).abs() <= 3 {
      println!("1");
      return
    }


  if (r1 - c1) % 2 == (r2 -c2) % 2 || (r1+c1) % 2 == (r2+c2) % 2 {
    println!("2");
    return
  }

  for i in -5..=5 {
    for j in -5..=5 {
      let nr = r1+i;
      let nc = c1+j;
      if (nr-r1).abs() + (nc-c1).abs() <= 3 {
        if nr+ nc == r2 + c2 ||
          nr - nc == r2 - c2 || 
          (nr-r2).abs() + (nc-c2).abs() <= 3 {
          println!("2");
          return
        }
      }
    }
  }

  println!("3");
}
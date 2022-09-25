/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn main() { 
  input! { 
    x:i32,
    y:i32,
    z:i32,
  }

  if x < 0 {
    if 0 < y || y < x {
      println!("{}", x.abs());
      return
    }
    if y < 0 {
      if z < y {
        println!("-1");
        return
      } else {
        if 0 < z {
          println!("{}", x.abs() + 2 * z.abs());
          return
        } else {
          println!("{}", x.abs());
          return
        }
      }
    }
  }
  else {
    if y < 0 || x < y {
      println!("{}", x.abs());
      return
    }
    if 0 < y {
      if y < z {
        println!("-1");
        return
      } else {
        if z < 0 {
          println!("{}", x.abs() + 2 * z.abs());
          return
        } else {
          println!("{}", x.abs());
          return
        }
      }
    }
  }
}
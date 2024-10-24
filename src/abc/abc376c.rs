/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut a:[isize;n],
    mut b:[isize;n-1]
  }

  a.sort();
  b.sort();
  let mut target = None;
  while let Some(v) = b.pop() {
    let lv = a.pop().unwrap();
    if v < lv {
      if target.is_none() {
        target = Some(lv);
        b.push(v);
      } else {
        println!("-1");
        return 
      }
    }
  }

  if target.is_none() && !a.is_empty() {
    println!("{}", a[0]);
  } else if let Some(v) = target {
    println!("{}", v);
  } else {
    println!("-1");
  }  
}
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
    a:[usize;n]
  }

  let mut map = HashMap::new();
  let mut result = vec![-1;n];

  for i in 0..n {
    if let Some(v) = map.get(&a[i]) {
      result[i] = *v as i32;
    }

    map.insert(a[i], i+1);
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
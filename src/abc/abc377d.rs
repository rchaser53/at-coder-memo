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
    m:usize,
    lr:[(usize,usize);n]
  }

  let mut result = 0;
  let mut lbset = BTreeSet::new();
  let mut rbset = BTreeSet::new();

  for i in 0..n {
    let (l,r) = lr[i];
    lbset.insert((l,i));
    rbset.insert((r,i));
  }

  for i in 1..=m {
    let rmin = if let Some((rv,_)) = rbset.iter().next() {
      *rv
    } else {
      m + 1
    };
    result += rmin - i;

    while !lbset.is_empty() {
      let (lmin, li) = *lbset.iter().next().unwrap();
      if i < lmin {
        break
      }

      lbset.remove(&(lmin,li));
      let rv = lr[li].1;
      rbset.remove(&(rv,li));
    }
  }

  println!("{}", result);
}
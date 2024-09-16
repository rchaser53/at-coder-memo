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
    a:[isize;n],
    m:usize,
    b:[isize;m],
    l:usize,
    c:[isize;l],
    q:usize,
    x:[isize;q]
  }

  let mut set = HashSet::new();
  for i in 0..n {
    for j in 0..m {
      for k in 0..l {
        set.insert(a[i]+b[j]+c[k]);
      }
    }
  }
  
  for v in x {
    if set.contains(&v) {
      println!("Yes");
    } else {
      println!("No");
    }
  }
  
}
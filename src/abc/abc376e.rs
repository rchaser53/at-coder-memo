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
    t:usize
  }

  for _ in 0..t {
    input! {
      n:usize,
      k:usize,
      a:[usize;n],
      b:[usize;n]
    }

    let mut x = vec![(0,0);n];
    for i in 0..n {
      x[i] = (a[i],b[i]);
    }
    x.sort_by(|a,b| {
      let v = a.0.cmp(&b.0);
      if v == Ordering::Equal {
        a.1.cmp(&b.1)
      } else {
        v
      }
    });

    let mut tot = 0;
    let mut btset = BTreeSet::new();
    for i in 0..k {
      tot += x[i].1;
      btset.insert((x[i].1, i));
    }

    let mut result = x[k-1].0 * tot;
    for i in k..n {
      let nv = x[i].1;
      btset.insert((x[i].1, i));

      let (rv, ri) = *btset.iter().rev().next().unwrap();
      btset.remove(&(rv,ri));
      tot = tot + nv - rv;
      result = result.min(tot * x[i].0);
    }

    println!("{}", result);
  }
}
/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn helper(a:Vec<Vec<usize>>) -> usize {
  let mut result = 0;
  for arr in a {
    let mut set = HashSet::new();
    let mut ri = 0;
    let n = arr.len();

    for li in 0..n {
      while ri < n && !set.contains(&arr[ri]) {
        set.insert(arr[ri]);
        ri += 1;
      }

      result = result.max(set.len() * 2);
      set.remove(&arr[li]);
    }

    result = result.max(set.len() * 2);
  }

  result
}

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  let mut odds = vec![vec![]];
  let mut evens = vec![vec![]];
  for i in (0..n-1).step_by(2) {
    let li = odds.len() - 1;
    if a[i] == a[i+1] {
      odds[li].push(a[i]);
    } else if !odds[li].is_empty() {
      odds.push(vec![]);
    }
  }

  for i in (1..n-1).step_by(2) {
    let li = evens.len() - 1;
    if a[i] == a[i+1] {
      evens[li].push(a[i]);
    } else if !evens[li].is_empty() {
      evens.push(vec![]);
    }
  }

  println!("{}", helper(odds).max(helper(evens)));
}
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
    x:[isize;n],
    p:[isize;n],
    q:usize,
    lr:[(isize,isize);q]
  }

  let mut set = HashSet::new();
  for &v in &x {
    set.insert(v);
  }
  for &(l,r) in &lr {
    set.insert(l);
    set.insert(r);
  }
  let mut arr = set.into_iter().collect::<Vec<_>>();
  arr.sort();

  let m = arr.len();
  let mut memo = vec![0;m+1];

  let mut map = HashMap::new();
  for i in 0..n {
    map.insert(x[i], i);
  }

  let mut dict = HashMap::new();
  for i in 0..m {
    memo[i+1] = memo[i];

    dict.insert(arr[i], i);
    if let Some(xi) = map.get(&arr[i]) {
      memo[i+1] += p[*xi];
    }
  }

  for (l, r) in lr {
    let li = *dict.get(&l).unwrap();
    let ri = *dict.get(&r).unwrap();

    println!("{}", memo[ri+1] - memo[li]);
  }
}